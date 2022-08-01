use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub(crate) struct EntityId(u64);

impl EntityId {
    pub fn initial() -> Self {
        Self(0)
    }

    pub fn next(&self) -> Self {
        Self(self.0 + 1)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct Entity {
    pub(crate) id: EntityId,
    pub(crate) components: HashMap<String, rhai::Dynamic>,
}

impl Entity {
    pub fn new(id: EntityId) -> Self {
        Self {
            id,
            components: Default::default(),
        }
    }
}
