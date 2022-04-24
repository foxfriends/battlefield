use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
#[serde(transparent)]
pub struct State(Value);

impl AsRef<Value> for State {
    fn as_ref(&self) -> &Value {
        &self.0
    }
}
