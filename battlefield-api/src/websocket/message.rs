use crate::Command;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    Identify(String),
    Sync,
    Command(Command),
}
