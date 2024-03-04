use crate::llm::{chat::{ChatMessage, ChatProvider}, tools::Tool};

pub struct GeminiChat {
    api_key: String,
}

impl GeminiChat {
    pub fn new(api_key: String) -> GeminiChat {
        GeminiChat { api_key }
    }
}

impl ChatProvider for GeminiChat {
    async fn chat(&self, messages: &Vec<ChatMessage>, tools: Option<Vec<&dyn Tool>>) -> Result<String, Box<dyn std::error::Error>> {
        Ok("Gemini chat: not implemented yet.".to_string())
    }
}
