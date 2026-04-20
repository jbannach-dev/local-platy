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

#[tauri::command]
fn prompt(text: String) -> String{
    return "hello world".to_string();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app|{
            let model_path = app
                .path()
                .resolve(
                    "models/model.gguf",
                    BaseDirectory::Resource,
                )
                .expect("Failed to find the model");

            let model = model::spawn_thread(model_path);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![prompt])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
