use serde_json::Value;

mod command;
mod scenario;
mod state;

pub use command::Command;
pub use scenario::Scenario;
pub use state::State;

pub fn process(
    _command: Command,
    _scenario: &Scenario,
    _state: &mut State,
) -> anyhow::Result<Value> {
    Ok(Value::default())
}
