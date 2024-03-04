use super::{ChatRequest, ChatResponse};

pub async fn chat(request: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error>> {
    let url = "https://api.openai.com/v1/chat/completions";
    let body = serde_json::to_string(&request)?;
    let auth_header = format!("Bearer {}", request.api_key);

    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("Authorization", auth_header)
        .body(body)
        .send()
        .await?;

    if response.status() != 200 {
        Err(response.text().await?.into())
    } else {
        let response_text = response.text().await?;
        let completion: ChatResponse = serde_json::from_str(&response_text)?;
        Ok(completion)
    }
}
