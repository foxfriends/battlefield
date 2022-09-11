use super::Game;
use crate::websocket::Update;
use actix::prelude::*;
use battlefield_core::State;
use json_patch::diff;

#[derive(Message)]
#[rtype(result = "anyhow::Result<()>")]
pub struct Commit(pub State);

impl Handler<Commit> for Game {
    type Result = MessageResult<Commit>;

    fn handle(&mut self, Commit(new_state): Commit, _ctx: &mut Self::Context) -> Self::Result {
        let old_state_json = serde_json::to_value(&self.game.state).unwrap();
        let new_state_json = serde_json::to_value(&new_state).unwrap();
        let patch = diff(&old_state_json, &new_state_json);
        self.game.state = new_state;

        for subscriber in &self.subscribers {
            if let Some(addr) = subscriber.upgrade() {
                addr.do_send(Update(patch.clone()));
            }
        }
        MessageResult(Ok(()))
    }
}
