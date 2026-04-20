//    Copyright 2026 J. Bannach

//    Licensed under the Apache License, Version 2.0 (the "License");
//    you may not use this file except in compliance with the License.
//    You may obtain a copy of the License at

//        http://www.apache.org/licenses/LICENSE-2.0

//    Unless required by applicable law or agreed to in writing, software
//    distributed under the License is distributed on an "AS IS" BASIS,
//    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//    See the License for the specific language governing permissions and
//    limitations under the License.

use std::thread;
use std::path::PathBuf;
use std::num::NonZeroU32;

use llama_cpp_2::llama_backend::LlamaBackend;
use llama_cpp_2::model::params::LlamaModelParams;
use llama_cpp_2::model::{Special, AddBos, LlamaModel};
use llama_cpp_2::context::params::LlamaContextParams;
use llama_cpp_2::llama_batch::LlamaBatch;
use llama_cpp_2::sampling::LlamaSampler;

use tokio::sync::{mpsc, oneshot};

pub struct ModelTask {
    pub text: String,
    pub response_tx: oneshot::Sender<String>,
}

pub struct ModelState {
    pub tx: mpsc::Sender<ModelTask>,
}

pub fn spawn_thread(
    model_path: PathBuf,
    context_size: u32
) -> mpsc::Sender<ModelTask>{
    let (tx, mut rx) = mpsc::channel::<ModelTask>(10);

    std::thread::spawn(move ||{

        //Load Model    
        let backend = LlamaBackend::init().unwrap();
        let model_params = LlamaModelParams::default();
        let model = LlamaModel::load_from_file(&backend, model_path, &model_params)
            .expect("Failed to load the model");    

        //Create context
        let mut seq_pos_y = 0;
        let ctx_params = LlamaContextParams::default().with_n_ctx(NonZeroU32::new(context_size)); 
        let mut ctx = model
            .new_context(&backend, ctx_params)
            .expect("Failed to create the context");

        while let Some(task) = rx.blocking_recv() {
            let recived_text = task.text;

            //Handle prompt logic
            let prompt = recived_text;
            let tokens = model.str_to_token(prompt.as_str(), AddBos::Never).unwrap();
            let mut batch = LlamaBatch::new(2048, 1);

            for (i, token) in tokens.iter().enumerate() {

                let _ = batch.add(
                    *token,
                    seq_pos_y as i32,
                    &[0][..],
                    i == tokens.len() - 1,
                );

                seq_pos_y+=1;
            }

            let _ = ctx.decode(&mut batch);
            let mut sampler = LlamaSampler::greedy();
            let mut n_cur = tokens.len() as i32;
            let n_len = 2048; // Maximum number of tokens to generate
            let mut output = String::from("");  

            while n_cur < n_len {
                let _ = ctx.decode(&mut batch);
                let token_id = sampler.sample(&ctx, batch.n_tokens() - 1);

                if token_id == model.token_eos() {
                    break;
                }

                output += &model.token_to_str(token_id, Special::Tokenize).unwrap();
                batch.clear();

                let _ = batch.add(token_id, seq_pos_y, &[0][..], true);
                seq_pos_y += 1;
                n_cur += 1;
            }

            let response = format!("{}", output);
            let _ = task.response_tx.send(output);
        }
    });

    tx
}
