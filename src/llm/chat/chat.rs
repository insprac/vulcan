use crate::llm::tools::Tool;

pub enum ChatRole {
    User,
    Assistant,
    System,
}

pub trait ChatProvider {
    async fn chat(
        &self,
        messages: &Vec<ChatMessage>,
        tools: Option<Vec<&dyn Tool>>,
    ) -> Result<String, Box<dyn std::error::Error>>;
}

pub struct ChatMessage {
    pub role: ChatRole,
    pub content: String,
}

impl ChatMessage {
    pub fn user(content: String) -> ChatMessage {
        ChatMessage {
            role: ChatRole::User,
            content,
        }
    }

    pub fn assistant(content: String) -> ChatMessage {
        ChatMessage {
            role: ChatRole::Assistant,
            content,
        }
    }

    pub fn system(content: String) -> ChatMessage {
        ChatMessage {
            role: ChatRole::System,
            content,
        }
    }
}
