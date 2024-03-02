use crate::llm::chat::{ChatMessage, ChatProvider};
use crate::llm::tools::Tool;

pub struct GPTChat {
    model: String,
    api_key: String,
}

impl GPTChat {
    pub fn new(api_key: String) -> GPTChat {
        let model = "gpt-3.5-turbo".to_string();
        GPTChat { api_key, model }
    }
}

impl ChatProvider for GPTChat {
    fn chat(&self, messages: &Vec<ChatMessage>, tools: Option<Vec<&dyn Tool>>) -> &str {
        "Open AI chat: not implemented yet."
    }
}
