use super::{ChatRequest, ChatResponse, EmbedRequest, EmbedResponse};

pub async fn chat(request: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let body = serde_json::to_string(&request)?;
    post(client, "chat/completions", &body, &request.api_key).await
}

pub async fn embed(request: EmbedRequest) -> Result<EmbedResponse, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let body = serde_json::json!({
        "model": request.model,
        "input": request.inputs,
        "encoding_format": "float",
    });
    post(client, "embeddings", &body.to_string(), &request.api_key).await
}

async fn post<T>(
    client: reqwest::Client,
    path: &str,
    body: &str,
    api_key: &str,
) -> Result<T, Box<dyn std::error::Error>>
where
    T: serde::de::DeserializeOwned,
{
    let response = client
        .post(url(path))
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .body(body.to_string())
        .send()
        .await?;

    handle_response(response).await
}

async fn handle_response<T>(response: reqwest::Response) -> Result<T, Box<dyn std::error::Error>>
where
    T: serde::de::DeserializeOwned,
{
    if response.status() != 200 {
        Err(response.text().await?.into())
    } else {
        let response_text = response.text().await?;
        let result: T = serde_json::from_str(&response_text)?;
        Ok(result)
    }
}

fn url(path: &str) -> String {
    format!("https://api.openai.com/v1/{}", path)
}
