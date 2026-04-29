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

use llama_cpp_2::context::LlamaContext;
use llama_cpp_2::model::LlamaModel;
use tokio::sync::oneshot;

use super::handlers;

pub enum ModelCommands {
    Prompt {
        text: String,
        response_tx: oneshot::Sender<String>,
    },
}

impl ModelCommands {
    pub fn exec(
        self,
        model: &LlamaModel,
        ctx: &mut LlamaContext,
        seq_id: u32,
        max_seq: i32,
        seq_pos_y: &mut i32,
        context_size: u32,
        system_prompt_tokens_position: i32,
    ) {
        match self {
            ModelCommands::Prompt { text, response_tx } => {
                handlers::handle_prompt(
                    model,
                    ctx,
                    seq_id,
                    max_seq,
                    seq_pos_y,
                    context_size,
                    system_prompt_tokens_position,
                    text,
                    response_tx,
                );
            }
        }
    }
}
