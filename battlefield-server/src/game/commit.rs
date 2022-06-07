use super::Game;
use crate::socket::Notification;
use actix::prelude::*;
use battlefield_core::State;
use json_patch::diff;

#[derive(Message)]
#[rtype(result = "anyhow::Result<()>")]
pub struct Commit(pub State);

impl Handler<Commit> for Game {
    type Result = ResponseFuture<anyhow::Result<()>>;

    fn handle(&mut self, Commit(new_state): Commit, _ctx: &mut Self::Context) -> Self::Result {
        let old_state_json = serde_json::to_value(&self.state).unwrap();
        let new_state_json = serde_json::to_value(&new_state).unwrap();
        let patch = diff(&old_state_json, &new_state_json);
        self.state = new_state;

        let state = self.state.clone();
        let engine = self.engine.clone();
        let scenario = self.scenario.clone();
        let subscribers = self.subscribers.clone();
        Box::pin(async move {
            let engine = engine.read().await;
            let actions = engine.commands(&scenario, &state)?;
            for subscriber in &subscribers {
                if let Some(addr) = subscriber.upgrade() {
                    addr.do_send(Notification::Update {
                        patch: patch.clone(),
                        actions: actions.clone(),
                    });
                }
            }
            Ok(())
        })
    }
}
