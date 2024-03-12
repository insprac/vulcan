use serde_json::Value;

#[derive(Debug, Clone)]
pub struct Tool {
    pub name: String,
    pub description: String,
    pub definition: Value,
}

#[derive(Debug, Clone)]
pub struct ToolCall {
    pub name: String,
    pub arguments: Value,
}
