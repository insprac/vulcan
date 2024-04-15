use crate::chat::{ChatMessage, ChatProvider};
use crate::tools::ToolDefinition;

pub struct GeminiChat {
    #[allow(dead_code)]
    api_key: String,
}

impl GeminiChat {
    pub fn new(api_key: String) -> GeminiChat {
        GeminiChat { api_key }
    }
}

#[async_trait::async_trait]
impl ChatProvider for GeminiChat {
    async fn chat(
        &self,
        _messages: &Vec<ChatMessage>,
        _tools: Vec<ToolDefinition>,
    ) -> crate::error::Result<ChatMessage> {
        let message = ChatMessage::assistant("Gemini chat: not implemented yet.".to_string());
        Ok(message)
    }
}
