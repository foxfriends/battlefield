use super::Module;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Scenario {
    pub name: String,
    pub description: String,
    pub map: String,
    pub modules: HashMap<String, Module>,
}
