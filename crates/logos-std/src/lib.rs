use serde_json::Value;

pub trait LogosNode {
    fn execute(&self, input: Value) -> Value;
}
