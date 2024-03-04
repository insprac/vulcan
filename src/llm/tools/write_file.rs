use serde_json::{json, Value};
use serde::{Deserialize, Serialize};

use super::Tool;

#[derive(Debug, Serialize, Deserialize)]
struct WriteFileToolArgs {
    pub path: String,
    pub content: String,
}

pub struct WriteFileTool {}

impl Tool for WriteFileTool {
    fn name(&self) -> String {
        "write_file".to_string()
    }

    fn description(&self) -> String {
        "Write content to a file on the local file system".to_string()
    }

    fn define(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "The relative path to the file to write to"
                },
                "content": {
                    "type": "string",
                    "description": "The content to write to the file"
                }
            }
        })
    }

    fn call(&self, args: Value) -> String {
        let args = serde_json::from_value::<WriteFileToolArgs>(args).unwrap();
        format!("Wrote to file: {}", args.path)
    }
}
