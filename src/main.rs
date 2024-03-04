use crate::llm::chat::{ChatMessage, ChatProvider};
use crate::llm::chat::providers::GPTChat;

mod env;
mod llm;
mod openai;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let openai = GPTChat::new(env::openai_api_key());
    let messages = vec![
        ChatMessage::system(
            "You are a tutor helping me learn about space.".to_string(),
        ),
        ChatMessage::user("How far is Jupiter from the sun?".to_string()),
    ];
    let result = openai.chat(&messages, None).await?;
    println!("{}", result);
    Ok(())
}
