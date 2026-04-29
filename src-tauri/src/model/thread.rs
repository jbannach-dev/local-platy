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
use tokio::sync::mpsc;

use tauri::Manager;
use tauri_plugin_fs::FsExt;

use llama_cpp_2::context::params::LlamaContextParams;
use llama_cpp_2::llama_backend::LlamaBackend;
use llama_cpp_2::llama_batch::LlamaBatch;
use llama_cpp_2::model::params::LlamaModelParams;
use llama_cpp_2::model::{AddBos, LlamaModel};

use super::commands::ModelCommands;

pub struct ModelState {
    pub tx: mpsc::Sender<ModelCommands>,
}

pub fn spawn_thread(
    app_handle: tauri::AppHandle,
    model_path: PathBuf,
    context_size: u32,
    system_prompt: String,
) -> mpsc::Sender<ModelCommands> {
    let (tx, mut rx) = mpsc::channel::<ModelCommands>(10);

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
            task.exec(
                &model,
                &mut ctx,
                seq_id,
                max_seq,
                &mut seq_pos_y,
                context_size,
                system_prompt_tokens_position,
            );
        }
    });

    tx
}
