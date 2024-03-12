use std::fmt::Display;

use crate::chat::{ChatMessage, ChatProvider, ChatRole};
use crate::openai;
use crate::tools::{Tool, ToolCall};

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
        tools: Vec<Tool>,
    ) -> Result<ChatMessage, Box<dyn std::error::Error>> {
        let messages = messages.iter().map(convert_message).collect();
        let tools: Vec<openai::ChatToolDefinition> =
            tools.clone().iter().map(|t| convert_tool(t.clone())).collect();

        let response = chat_request(
            self.api_key.clone(),
            self.model.clone(),
            messages,
            self.temperature,
            tools,
        ).await?;

        let content = match response.choices[0].message.content {
            Some(ref content) => content.clone(),
            None => "".to_string(),
        };
        let tool_calls = match response.choices[0].message.tool_calls {
            Some(ref calls) => Some(calls.iter().map(convert_tool_call).collect()),
            None => None,
        };

        let response_message = ChatMessage {
            role: ChatRole::Assistant,
            content,
            tool_calls,
        };

        Ok(response_message)
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

async fn chat_request(
    api_key: String,
    model: String,
    messages: Vec<openai::ChatMessage>,
    temperature: f32,
    tools: Vec<openai::ChatToolDefinition>,
) -> Result<openai::ChatResponse, Box<dyn std::error::Error>> {
    let request = openai::ChatRequest {
        api_key,
        model,
        messages,
        temperature,
        tools: if tools.is_empty() { None } else { Some(tools) },
        tool_choice: None,
    };

    openai::chat(request).await
}

fn convert_message(message: &ChatMessage) -> openai::ChatMessage {
    openai::ChatMessage {
        role: message.role.to_string(),
        content: Some(message.content.clone()),
        tool_calls: None,
        tool_call_id: None,
    }
}

fn convert_tool(tool: Tool) -> openai::ChatToolDefinition {
    openai::ChatToolDefinition {
        tool_type: "function".to_string(),
        function: openai::ChatToolFunctionDefinition {
            name: tool.name,
            description: Some(tool.description),
            parameters: tool.params,
        },
    }
}

fn convert_tool_call(call: &openai::ChatToolCall) -> ToolCall {
    ToolCall {
        name: call.function.name.clone(),
        args: serde_json::to_value(call.function.arguments.clone()).unwrap(),
    }
}
