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

use std::num::NonZeroU32;
use std::path::PathBuf;
use std::thread;

use tauri::Manager;
use tauri_plugin_fs::FsExt;

use llama_cpp_2::context::params::LlamaContextParams;
use llama_cpp_2::llama_backend::LlamaBackend;
use llama_cpp_2::llama_batch::LlamaBatch;
use llama_cpp_2::model::params::LlamaModelParams;
use llama_cpp_2::model::{AddBos, LlamaModel};
use llama_cpp_2::sampling::LlamaSampler;

use rand::Rng;

use encoding_rs::UTF_8;

use tokio::sync::{mpsc, oneshot};

pub struct ModelTask {
    pub text: String,
    pub response_tx: oneshot::Sender<String>,
}

pub struct ModelState {
    pub tx: mpsc::Sender<ModelTask>,
}

pub fn spawn_thread(
    app_handle: tauri::AppHandle,
    model_path: PathBuf,
    context_size: u32,
    system_prompt: String,
) -> mpsc::Sender<ModelTask> {
    let (tx, mut rx) = mpsc::channel::<ModelTask>(10);

    thread::spawn(move || {
        let finnal_path;

        //On Android, copy the model to internal storage to ensure the application has access to it.
        if cfg!(target_os = "android") {
            let app_data_model_path = app_handle.path().app_data_dir().unwrap().join("model.gguf");
            if !app_data_model_path.exists() {
                let _ = std::fs::create_dir_all(app_data_model_path.parent().unwrap());
                match app_handle.fs().read(&model_path) {
                    Ok(bytes) => {
                        if let Err(e) = std::fs::write(&app_data_model_path, bytes) {
                            eprintln!("Error: {}", e);
                        }
                    }
                    Err(e) => eprintln!("Error during reading: {}", e),
                }
            }

            finnal_path = app_data_model_path;
        } else {
            finnal_path = model_path;
        }

        //Current sequence id
        let seq_id = 0;

        // Maximum number of tokens to generate in a single sequence.
        let max_seq: i32 = 2024;

        //Load Model
        let backend = LlamaBackend::init().unwrap();
        let model_params = LlamaModelParams::default();
        let model = LlamaModel::load_from_file(&backend, finnal_path, &model_params)
            .expect("Failed to load the model");

        //Create context
        let mut seq_pos_y = 0;
        let ctx_params = LlamaContextParams::default().with_n_ctx(NonZeroU32::new(context_size));
        let mut ctx = model
            .new_context(&backend, ctx_params)
            .expect("Failed to create the context");

        //Add system prompt
        let mut system_prompt_finnal = String::from("");
        system_prompt_finnal += "<|im_start|>system\n";
        system_prompt_finnal += &system_prompt;
        system_prompt_finnal += "<|im_end|>\n";

        let system_tokens = model
            .str_to_token(system_prompt.as_str(), AddBos::Never)
            .unwrap();
        let mut system_prompt_batch = LlamaBatch::new(2048, 1);
        for (i, system_token) in system_tokens.iter().enumerate() {
            let _ = system_prompt_batch.add(
                *system_token,
                seq_pos_y as i32,
                &[0][..],
                i == system_tokens.len() - 1,
            );

            seq_pos_y += 1;
        }

        let system_prompt_tokens_position = seq_pos_y - 1;

        let _ = ctx.decode(&mut system_prompt_batch);

        while let Some(task) = rx.blocking_recv() {
            let recived_text = task.text;

            //Handle prompt logic
            let mut prompt = String::from("");
            prompt += "<|im_start|>user\n";
            prompt += &recived_text;
            prompt += "<|im_end|>\n";
            prompt += "<|im_start|>assistant\n";
            prompt += "<think>";

            let tokens = model.str_to_token(prompt.as_str(), AddBos::Never).unwrap();
            let mut batch = LlamaBatch::new(max_seq as usize, 1);
            for (i, token) in tokens.iter().enumerate() {
                let _ = batch.add(*token, seq_pos_y as i32, &[0][..], i == tokens.len() - 1);
                seq_pos_y += 1;
            }
            let _ = ctx.decode(&mut batch);

            //Sampler
            let mut rng = rand::thread_rng();
            let seed: u32 = rng.gen();

            let mut sampler = LlamaSampler::chain_simple([
                LlamaSampler::penalties(64, 1.1, 0.0, 2.0),
                LlamaSampler::temp(0.5),
                LlamaSampler::min_p(0.05, 1),
                LlamaSampler::dist(seed),
            ]);

            let mut n_cur = tokens.len() as i32;
            let mut output = String::from("");
            let mut decoder = UTF_8.new_decoder();

            while n_cur < max_seq {
                //Sliding window implementation
                let current_seq_pos = (seq_pos_y) as u32;

                if context_size < current_seq_pos {
                    // Set the deletion size to 25% of the maximum context capacity.
                    let mut delete_by = (context_size as f32 * 0.25).round() as i32;
                    let difference = (current_seq_pos - context_size) as i32;

                    // Include the overflowed context in the deletion range if there is any.
                    if difference > 0 {
                        delete_by += difference;
                    }

                    // Set parameters for the upcoming shift
                    let tokens_to_keep = system_prompt_tokens_position as u32;
                    let tokens_to_discard = (system_prompt_tokens_position + delete_by) as u32;
                    let current_seq_position = seq_pos_y as u32;
                    let shift_by = delete_by as i32;
                    let new_seq_position = (seq_pos_y - delete_by) as u32;

                    // Clear the KV cache while preserving the system prompt.
                    ctx.clear_kv_cache_seq(
                        Some(seq_id),
                        Some(tokens_to_keep),
                        Some(tokens_to_discard),
                    )
                    .unwrap();

                    // Shift the remaining KV cache forward to close the gap after the system prompt.
                    ctx.kv_cache_seq_add(
                        seq_id as i32,
                        Some(tokens_to_discard),
                        Some(current_seq_position),
                        -shift_by,
                    )
                    .unwrap();

                    // Clear the end of the KV cache to remove duplicate context
                    ctx.clear_kv_cache_seq(Some(seq_id), Some(new_seq_position), None)
                        .unwrap();

                    seq_pos_y = (new_seq_position) as i32;
                }

                let token_id = sampler.sample(&ctx, batch.n_tokens() - 1);

                if token_id == model.token_eos() {
                    break;
                }

                if let Ok(piece) = model.token_to_piece(token_id, &mut decoder, false, None) {
                    output += &piece;
                } else {
                    eprintln!("Warning: undefined Token-Type at ID {}", token_id);
                }

                batch.clear();

                let _ = batch.add(token_id, seq_pos_y, &[0][..], true);
                let _ = ctx.decode(&mut batch);
                seq_pos_y += 1;
                n_cur += 1;
            }
            let _ = task.response_tx.send(output);
        }
    });

    tx
}
