use crate::chat::{ChatMessage, ChatProvider};
use crate::tools::Tool;

pub struct GeminiChat {
    api_key: String,
}

impl GeminiChat {
    pub fn new(api_key: String) -> GeminiChat {
        GeminiChat { api_key }
    }
}

impl ChatProvider for GeminiChat {
    async fn chat(
        &self,
        _messages: &Vec<ChatMessage>,
        _tools: Vec<Tool>,
    ) -> Result<ChatMessage, Box<dyn std::error::Error>> {
        let message = ChatMessage::assistant("Gemini chat: not implemented yet.".to_string());
        Ok(message)
    }
}
