use serde_json::Value;

mod message;
mod state;

pub use message::Message;
pub use state::State;

pub fn process(_message: Message, _state: &mut State) -> anyhow::Result<Value> {
    Ok(Value::default())
}
