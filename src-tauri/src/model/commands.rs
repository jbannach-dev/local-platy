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
