# Vulcan Chat Assistant

Vulcan is an LLM chat assistant rust library supporting different providers and tools.

## Usage

```bash
cargo add vulcan --git https://github.com/insprac/vulcan.git
```

### Example: Using GPTChat

```rust
use vulcan::chat::{ChatMessage, ChatProvider};
use vulcan::chat::providers::GPTChat;

#[tokio::main]
async fn main() {
    let api_key = "your_openai_api_key".to_string();
    let model = "gpt-4-turbo".to_string();
    let temperature = 0.7;

    let chat_provider = GPTChat::new(model, api_key, temperature);
    let prompt = "Hello, how can I assist you today?".to_string();
    let messages = vec![ChatMessage::user(prompt)];

    match chat_provider.chat(&messages, vec![]).await {
        Ok(response) => println!("Response: {}", response.content),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```
