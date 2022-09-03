use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Module {
    pub name: String,
    pub version: String,
    pub config: serde_json::Value,
}
