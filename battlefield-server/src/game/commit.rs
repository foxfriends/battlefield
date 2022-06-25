use super::Game;
use crate::websocket::Notification;
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

        let actions = match self.engine.commands(&self.game.scenario, &self.game.state) {
            Ok(actions) => actions,
            Err(error) => return MessageResult(Err(error.into())),
        };
        for subscriber in &self.subscribers {
            if let Some(addr) = subscriber.upgrade() {
                addr.do_send(Notification::Update {
                    patch: patch.clone(),
                    actions: actions.clone(),
                });
            }
        }
        MessageResult(Ok(()))
    }
}
