use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::tools::{ToolDefinition, ToolCall};

pub trait ChatProvider {
    async fn chat(
        &self,
        messages: &Vec<ChatMessage>,
        tools: Vec<ToolDefinition>,
    ) -> Result<ChatMessage, Box<dyn std::error::Error>>;
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ChatRole {
    User,
    Assistant,
    System,
    Tool,
}

impl Display for ChatRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChatRole::User => write!(f, "user"),
            ChatRole::Assistant => write!(f, "assistant"),
            ChatRole::System => write!(f, "system"),
            ChatRole::Tool => write!(f, "tool"),
        }
    }
}

impl ChatRole {
    pub fn from_string(role: &str) -> Result<ChatRole, Box<dyn std::error::Error>> {
        match role {
            "user" => Ok(ChatRole::User),
            "assistant" => Ok(ChatRole::Assistant),
            "system" => Ok(ChatRole::System),
            "tool" => Ok(ChatRole::Tool),
            _ => Err(format!("Unknown role: {}", role).into()),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChatMessage {
    pub role: ChatRole,
    pub content: String,
    pub tool_calls: Option<Vec<ToolCall>>,
    pub tool_call_id: Option<String>,
}

impl ChatMessage {
    pub fn user(content: String) -> ChatMessage {
        ChatMessage {
            role: ChatRole::User,
            content,
            tool_calls: None,
            tool_call_id: None,
        }
    }

    pub fn assistant(content: String) -> ChatMessage {
        ChatMessage {
            role: ChatRole::Assistant,
            content,
            tool_calls: None,
            tool_call_id: None,
        }
    }

    pub fn system(content: String) -> ChatMessage {
        ChatMessage {
            role: ChatRole::System,
            content,
            tool_calls: None,
            tool_call_id: None,
        }
    }

    pub fn tool(id: String, content: String) -> ChatMessage {
        ChatMessage {
            role: ChatRole::Tool,
            content,
            tool_calls: None,
            tool_call_id: Some(id),
        }
    }
}

impl Display for ChatMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]\n{}", self.role, self.content)
    }
}
