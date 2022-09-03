use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Hash, Serialize, Deserialize, Debug)]
pub struct Command(pub String);
