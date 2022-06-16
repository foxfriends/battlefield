use super::SocketHandler;
use actix::prelude::*;
use battlefield_core::{data::Scenario, Command, State};
use json_patch::Patch;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Message)]
#[rtype(result = "()")]
pub enum Notification {
    Init {
        id: Uuid,
        scenario: Scenario,
        state: State,
        actions: Vec<Command>,
    },
    Sync {
        state: State,
        actions: Vec<Command>,
    },
    Update {
        patch: Patch,
        actions: Vec<Command>,
    },
    Err(String),
}

impl Notification {
    pub fn error(err: impl std::fmt::Display) -> Self {
        Notification::Err(err.to_string())
    }
}

impl Handler<Notification> for SocketHandler {
    type Result = ();

    fn handle(&mut self, notification: Notification, ctx: &mut Self::Context) {
        ctx.text(serde_json::to_string(&notification).unwrap());
    }
}
