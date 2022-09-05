use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Deserialize, Serialize, Debug)]
pub struct Module {
    pub name: String,
    pub version: String,
    pub config: serde_json::Value,
}
