use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ComponentType(String);

impl ComponentType {
    pub(crate) fn new(name: String) -> Self {
        Self(name)
    }
}
