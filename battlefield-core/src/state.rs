use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
#[serde(transparent)]
pub struct State(Value);
