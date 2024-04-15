extern crate vulcan;

use std::env::args;

use vulcan::chat::{ChatMessage, ChatProvider};
use vulcan::chat::providers::GPTChat;

#[tokio::main]
async fn main() -> vulcan::error::Result<()> {
    let args = args().collect::<Vec<String>>();
    if args.len() != 4 {
        panic!("Usage: chat <provider> <model> <prompt>");
    }
    let chat_provider = chat_provider(args[1].clone(), args[2].clone());
    let prompt = args[3].clone();
    let messages = vec![ChatMessage::user(prompt)];
    let result = chat_provider.chat(&messages, vec![]).await?;
    println!("{:?}", result);
    Ok(())
}

fn chat_provider (param: String, model: String) -> GPTChat {
    match param.as_str() {
        "openai" => {
            let api_key = std::env::var("OPENAI_API_KEY").unwrap();
            let temperature = 0.3;
            GPTChat::new(model.to_string(), api_key, temperature)
        },
        _ => panic!("Unknown chat provider")
    }
}
