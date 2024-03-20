use std::fmt::Display;

use crate::tools::{Tool, ToolCall};

#[derive(Clone, Debug)]
pub enum ChatRole {
    User,
    Assistant,
    System,
}

pub trait ChatProvider {
    async fn chat(
        &self,
        messages: &Vec<ChatMessage>,
        tools: Vec<Tool>,
    ) -> Result<ChatMessage, Box<dyn std::error::Error>>;
}

#[derive(Clone, Debug)]
pub struct ChatMessage {
    pub role: ChatRole,
    pub content: String,
    pub tool_calls: Option<Vec<ToolCall>>,
}

impl ChatMessage {
    pub fn user(content: String) -> ChatMessage {
        ChatMessage {
            role: ChatRole::User,
            content,
            tool_calls: None,
        }
    }

    pub fn assistant(content: String) -> ChatMessage {
        ChatMessage {
            role: ChatRole::Assistant,
            content,
            tool_calls: None,
        }
    }

    pub fn system(content: String) -> ChatMessage {
        ChatMessage {
            role: ChatRole::System,
            content,
            tool_calls: None,
        }
    }
}

impl Display for ChatRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChatRole::User => write!(f, "user"),
            ChatRole::Assistant => write!(f, "assistant"),
            ChatRole::System => write!(f, "system"),
        }
    }
}

impl Display for ChatMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]\n{}", self.role, self.content)
    }
}
