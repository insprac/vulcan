use serde_json::Value;

pub trait Tool {
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn define(&self) -> Value;
    fn call(&self, args: Value) -> String;
}

