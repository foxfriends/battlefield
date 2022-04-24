use serde_json::Value;

mod command;
mod state;

pub use command::Command;
pub use state::State;

pub fn process(_command: Command, _state: &mut State) -> anyhow::Result<Value> {
    Ok(Value::default())
}
