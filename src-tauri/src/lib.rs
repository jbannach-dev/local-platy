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

mod model;

use tauri::path::BaseDirectory;
use tauri::Manager;

use tokio::sync::oneshot;

use model::{ModelState, ModelTask};

#[tauri::command]
async fn prompt(text: String, state: tauri::State<'_, ModelState>) -> Result<String, String> {
    let (res_tx, res_rx) = oneshot::channel();

    state
        .tx
        .send(ModelTask {
            text,
            response_tx: res_tx,
        })
        .await
        .map_err(|_| "Worker disconnected")?;

    res_rx.await.map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            let model_path = app
                .path()
                .resolve("models/model.gguf", BaseDirectory::Resource)
                .expect("Failed to find the model");

            let context_size = 8192;
            let system_prompt = "You are a helpful assistant.".to_string();

            let tx = model::spawn_thread(
                app.handle().clone(),
                model_path,
                context_size,
                system_prompt,
            );
            app.manage(ModelState { tx });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![prompt])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
