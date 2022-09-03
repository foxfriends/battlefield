use battlefield_api as api;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Command(pub(crate) String);

impl Into<api::Command> for Command {
    fn into(self) -> api::Command {
        api::Command(self.0)
    }
}
