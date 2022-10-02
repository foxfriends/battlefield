use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub struct Player {
    pub id: usize,
    pub name: String,
}
