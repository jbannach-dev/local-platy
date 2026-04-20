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
use llama_cpp_2::llama_backend::LlamaBackend;
use llama_cpp_2::model::params::LlamaModelParams;
use llama_cpp_2::model::LlamaModel;

// Public wrapper that spawns the thread and returns the handle
pub fn spawn_thread(model_path: PathBuf) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        //Load Model
        let backend = LlamaBackend::init().unwrap();
        let model_params = LlamaModelParams::default();
        let model = LlamaModel::load_from_file(&backend, model_path, &model_params)
            .expect("Failed to load the model");    

        //Create Context
        let ctx_params = LlamaContextParams::default().with_n_ctx(NonZeroU32::new(512));
        let mut ctx = model
            .new_context(&backend, ctx_params)
            .expect("Failed to create the context");
    })
}
