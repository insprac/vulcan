use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::chat::ChatRole;

#[derive(Debug, Serialize)]
pub struct ChatRequest {
    #[serde(skip)]
    pub api_key: String,
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub temperature: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<ChatToolDefinition>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<Value>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ChatMessage {
    pub role: String,
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ChatToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
}

impl ChatMessage {
    pub fn from_vulcan_message(message: &crate::chat::ChatMessage) -> Self {
        let tool_calls: Option<Vec<ChatToolCall>> = match message.tool_calls {
            Some(ref calls) => Some(calls.iter().map(ChatToolCall::from_vulcan_call).collect()),
            None => None,
        };
        ChatMessage {
            role: message.role.to_string(),
            content: Some(message.content.clone()),
            tool_calls,
            tool_call_id: message.tool_call_id.clone(),
        }
    }

    pub fn to_vulcan_message(&self) -> crate::chat::ChatMessage {
        let tool_calls: Option<Vec<crate::tools::ToolCall>> = match self.tool_calls {
            Some(ref calls) => Some(calls.iter().map(ChatToolCall::to_vulcan_call).collect()),
            None => None,
        };
        crate::chat::ChatMessage {
            role: ChatRole::from_str(&self.role).unwrap_or(ChatRole::User),
            content: self.content.clone().unwrap_or("".to_string()),
            tool_calls,
            tool_call_id: self.tool_call_id.clone(),
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct ChatToolDefinition {
    #[serde(rename = "type")]
    pub tool_type: String,
    pub function: ChatToolFunctionDefinition,
}

impl ChatToolDefinition {
    pub fn from_vulcan_tool(tool: &crate::tools::ToolDefinition) -> Self {
        ChatToolDefinition {
            tool_type: "function".to_string(),
            function: ChatToolFunctionDefinition {
                name: tool.name.clone(),
                description: Some(tool.description.clone()),
                parameters: tool.params.clone(),
            },
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct ChatToolFunctionDefinition {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub parameters: Value,
}

#[derive(Debug, Deserialize)]
pub struct ChatResponse {
    pub id: String,
    pub model: String,
    pub choices: Vec<ChatChoice>,
    pub usage: ChatUsage,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ChatChoice {
    pub message: ChatMessage,
    pub finish_reason: String,
}

#[derive(Debug, Deserialize)]
pub struct ChatUsage {
    pub prompt_tokens: i32,
    pub completion_tokens: i32,
    pub total_tokens: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatToolCall {
    pub id: String,
    #[serde(rename = "type")]
    pub tool_type: String,
    pub function: ChatToolFunctionCall,
}

impl ChatToolCall {
    pub fn from_vulcan_call(call: &crate::tools::ToolCall) -> Self {
        ChatToolCall {
            id: call.id.clone(),
            tool_type: "function".to_string(),
            function: ChatToolFunctionCall {
                name: call.name.clone(),
                arguments: serde_json::to_string(&call.args).unwrap(),
            },
        }
    }

    pub fn to_vulcan_call(&self) -> crate::tools::ToolCall {
        crate::tools::ToolCall {
            id: self.id.clone(),
            name: self.function.name.clone(),
            args: serde_json::from_str(&self.function.arguments).unwrap(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatToolFunctionCall {
    pub name: String,
    pub arguments: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbedRequest {
    #[serde(skip)]
    pub api_key: String,
    pub model: String,
    pub inputs: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct EmbedResponse {
    pub data: Vec<Embedding>,
    pub model: String,
    pub usage: EmbedUsage,
}

#[derive(Debug, Deserialize)]
pub struct Embedding {
    pub embedding: Vec<f32>,
    pub index: u32,
}

#[derive(Debug, Deserialize)]
pub struct EmbedUsage {
    pub prompt_tokens: u32,
    pub total_tokens: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chat::{ChatMessage as VulcanChatMessage, ChatRole};
    use crate::tools::ToolCall as VulcanToolCall;
    use serde_json::json;

    #[test]
    fn test_from_vulcan_message() {
        let vulcan_message = VulcanChatMessage {
            role: ChatRole::User,
            content: "Hello, world!".to_string(),
            tool_calls: None,
            tool_call_id: None,
        };

        let chat_message = ChatMessage::from_vulcan_message(&vulcan_message);

        assert_eq!(chat_message.role, "user");
        assert_eq!(chat_message.content, Some("Hello, world!".to_string()));
        assert!(chat_message.tool_calls.is_none());
        assert!(chat_message.tool_call_id.is_none());
    }

    #[test]
    fn test_to_vulcan_message() {
        let chat_message = ChatMessage {
            role: "assistant".to_string(),
            content: Some("Hello, world!".to_string()),
            tool_calls: None,
            tool_call_id: None,
        };

        let vulcan_message = chat_message.to_vulcan_message();

        assert_eq!(vulcan_message.role.to_string(), "assistant");
        assert_eq!(vulcan_message.content, "Hello, world!");
        assert!(vulcan_message.tool_calls.is_none());
        assert!(vulcan_message.tool_call_id.is_none());
    }

    #[test]
    fn test_to_vulcan_message_no_content() {
        let chat_message = ChatMessage {
            role: "assistant".to_string(),
            content: None,
            tool_calls: None,
            tool_call_id: None,
        };

        let vulcan_message = chat_message.to_vulcan_message();

        assert_eq!(vulcan_message.role.to_string(), "assistant");
        assert_eq!(vulcan_message.content, "");
        assert!(vulcan_message.tool_calls.is_none());
        assert!(vulcan_message.tool_call_id.is_none());
    }
    #[test]
    fn test_from_vulcan_call() {
        let vulcan_call = VulcanToolCall {
            id: "123".to_string(),
            name: "test_function".to_string(),
            args: json!({"arg1": "value1"}),
        };

        let chat_tool_call = ChatToolCall::from_vulcan_call(&vulcan_call);

        assert_eq!(chat_tool_call.id, "123");
        assert_eq!(chat_tool_call.tool_type, "function");
        assert_eq!(chat_tool_call.function.name, "test_function");
        assert_eq!(chat_tool_call.function.arguments, "{\"arg1\":\"value1\"}");
    }

    #[test]
    fn test_to_vulcan_call() {
        let chat_tool_call = ChatToolCall {
            id: "123".to_string(),
            tool_type: "function".to_string(),
            function: ChatToolFunctionCall {
                name: "test_function".to_string(),
                arguments: "{\"arg1\":\"value1\"}".to_string(),
            },
        };

        let vulcan_call = chat_tool_call.to_vulcan_call();

        assert_eq!(vulcan_call.id, "123");
        assert_eq!(vulcan_call.name, "test_function");
        assert_eq!(vulcan_call.args, json!({"arg1": "value1"}));
    }
}
