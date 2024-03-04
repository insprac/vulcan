use std::fmt::Display;

use serde_json::Value;

use crate::chat::{ChatMessage, ChatProvider, ChatRole};
use crate::tools::Tool;
use crate::openai;

pub struct GPTChat {
    pub model: String,
    pub temperature: f32,
    api_key: String,
}

impl GPTChat {
    pub fn new(model: String, api_key: String, temperature: f32) -> GPTChat {
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
        tools: Vec<&dyn Tool>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let messages = messages.iter().map(convert_message).collect();
        let tool_definitions: Vec<openai::ChatToolDefinition> = tools.clone().iter().map(|t| convert_tool(*t)).collect();
        let request = openai::ChatRequest {
            api_key: self.api_key.clone(),
            model: self.model.clone(),
            messages,
            temperature: self.temperature,
            tools: if tool_definitions.is_empty() { None } else { Some(tool_definitions) },
            tool_choice: None,
        };
        let response = openai::chat(request).await?;

        let choice = response.choices[0].clone();

        if choice.message.tool_calls.is_some() {
            let tool_calls = choice.message.tool_calls.unwrap();
            for tool_call in tool_calls {
                for tool in tools.clone() {
                    if tool_call.function.name == tool.name() {
                        let args: Value = serde_json::from_str(&tool_call.function.arguments)?;
                        let result = tool.call(args);
                        return Ok(result);
                    }
                }
            }
        }

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
        tool_type: "function".to_string(),
        function: openai::ChatToolFunctionDefinition {
            name: tool.name(),
            description: Some(tool.description()),
            parameters: serde_json::Value::Null,
        },
    }
}
