use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use super::Tool;

#[derive(Debug, Serialize, Deserialize)]
struct SearchToolArgs {
    pub query: String,
}

pub struct SearchTool {}

impl Tool for SearchTool {
    fn name(&self) -> String {
        "search".to_string()
    }

    fn description(&self) -> String {
        "Search the web for new or more up-to-date information".to_string()
    }

    fn define(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "query": {
                    "type": "string",
                    "description": "The query term to search for"
                }
            },
            "required": ["query"]
        })
    }

    fn call(&self, args: Value) -> String {
        let args = serde_json::from_value::<SearchToolArgs>(args).unwrap();
        format!("Searching for: {}", args.query)
    }
}
