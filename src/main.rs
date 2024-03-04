use crate::llm::chat::{ChatMessage, ChatProvider};
use crate::llm::chat::providers::GPTChat;
use crate::llm::tools::{Tool, SearchTool};

mod llm;
mod openai;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let model = "gpt-4-turbo-preview".to_string();
    let api_key = std::env::var("OPENAI_API_KEY").unwrap();
    let temperature = 0.3;
    let openai = GPTChat::new(model, api_key, temperature);

    let tools: Vec<&dyn Tool> = vec![
        &SearchTool {},
    ];

    let messages = vec![
        ChatMessage::system(
            "You are a search assistant, take a user's request and search for the necessary information".to_string(),
        ),
        ChatMessage::user("Find the best vacum cleaner to buy in 2024".to_string()),
    ];

    let result = openai.chat(&messages, tools).await?;

    println!("{}", result);

    Ok(())
}
