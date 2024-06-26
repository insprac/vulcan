use crate::chat::{ChatMessage, ChatProvider};
use crate::openai;
use crate::tools::ToolDefinition;

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

#[async_trait::async_trait]
impl ChatProvider for GPTChat {
    async fn chat(
        &self,
        messages: &Vec<ChatMessage>,
        tools: Vec<ToolDefinition>,
    ) -> crate::error::Result<ChatMessage> {
        let messages = messages
            .iter()
            .map(openai::ChatMessage::from_vulcan_message)
            .collect();
        let tools: Vec<openai::ChatToolDefinition> = tools
            .clone()
            .iter()
            .map(openai::ChatToolDefinition::from_vulcan_tool)
            .collect();

        let response = chat_request(
            self.api_key.clone(),
            self.model.clone(),
            messages,
            self.temperature,
            tools,
        )
        .await?;

        let message = response.choices[0].message.to_vulcan_message();
        Ok(message)
    }
}

async fn chat_request(
    api_key: String,
    model: String,
    messages: Vec<openai::ChatMessage>,
    temperature: f32,
    tools: Vec<openai::ChatToolDefinition>,
) -> crate::error::Result<openai::ChatResponse> {
    let request = openai::ChatRequest {
        api_key,
        model,
        messages,
        temperature,
        tools: if tools.is_empty() { None } else { Some(tools) },
        tool_choice: None,
    };

    openai::chat(request)
        .await
        .map_err(|e| crate::error::Error::Provider(e.to_string()))
}
