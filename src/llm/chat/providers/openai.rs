use std::fmt::Display;

use crate::llm::chat::{ChatMessage, ChatProvider, ChatRole};
use crate::llm::tools::Tool;
use crate::openai;

pub struct GPTChat {
    pub model: String,
    pub temperature: f32,
    api_key: String,
}

impl GPTChat {
    pub fn new(api_key: String) -> GPTChat {
        let model = "gpt-3.5-turbo".to_string();
        let temperature = 0.3;
        GPTChat {
            api_key,
            model,
            temperature,
        }
    }
}

impl ChatProvider for GPTChat {
    async fn chat(
        &self,
        messages: &Vec<ChatMessage>,
        tools: Option<Vec<&dyn Tool>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let messages = messages.iter().map(convert_message).collect();
        let tools: Option<Vec<openai::ChatToolDefinition>> = match tools {
            Some(tools) => Some(tools.iter().map(|t| convert_tool(*t)).collect()),
            None => None,
        };
        let request = openai::ChatRequest {
            api_key: self.api_key.clone(),
            model: self.model.clone(),
            messages,
            temperature: self.temperature,
            tools,
            tool_choice: None,
        };
        let response = openai::chat(request).await?;
        let response = 
            match response.choices[0].message.content {
                Some(ref content) => content.clone(),
                None => "".to_string(),
            };
        Ok(response)
    }
}

impl Display for ChatRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChatRole::System => write!(f, "system"),
            ChatRole::User => write!(f, "user"),
            ChatRole::Assistant => write!(f, "assistant"),
        }
    }
}

fn convert_message(message: &ChatMessage) -> openai::ChatMessage {
    openai::ChatMessage {
        role: message.role.to_string(),
        content: Some(message.content.clone()),
        tool_calls: None,
        tool_call_id: None,
    }
}

fn convert_tool(tool: &dyn Tool) -> openai::ChatToolDefinition {
    openai::ChatToolDefinition {
        tool_type: "tool".to_string(),
        function: openai::ChatToolFunctionDefinition {
            name: tool.name(),
            description: Some(tool.description()),
            parameters: serde_json::Value::Null,
        },
    }
}
