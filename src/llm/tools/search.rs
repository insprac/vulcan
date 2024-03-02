use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use super::Tool;

#[derive(Debug, Serialize, Deserialize)]
struct SearchToolArgs {
    pub query: String,
}

pub struct SearchTool {}

impl Tool for SearchTool {
    fn define(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "query": {
                    "type": "string",
                    "description": "The query term to search for"
                }
            }
        })
    }

    fn call(&self, args: Value) -> String {
        let args = serde_json::from_value::<SearchToolArgs>(args).unwrap();
        format!("Searching for: {}", args.query)
    }
}
