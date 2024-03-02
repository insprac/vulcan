use serde_json::Value;

pub trait Tool {
    fn define(&self) -> Value;
    fn call(&self, args: Value) -> String;
}

