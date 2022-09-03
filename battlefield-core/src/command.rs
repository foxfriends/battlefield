use battlefield_api as api;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Command(pub(crate) String);

impl From<Command> for api::Command {
    fn from(command: Command) -> Self {
        Self(command.0)
    }
}
