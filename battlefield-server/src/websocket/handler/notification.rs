use super::SocketHandler;
use actix::prelude::*;
use battlefield_api::websocket as api;
use battlefield_core::{data::Scenario, Command, State};
use json_patch::Patch;
use uuid::Uuid;

#[derive(Message)]
#[rtype(result = "()")]
pub(crate) struct Notification(pub api::Notification);

impl Notification {
    pub fn init(id: Uuid, scenario: Scenario, state: State, commands: Vec<Command>) -> Self {
        Self(api::Notification::Init {
            id,
            scenario: scenario.into(),
            state: state.into(),
            commands: commands.into_iter().map(Into::into).collect(),
        })
    }
    pub fn update(patch: Patch, commands: Vec<Command>) -> Self {
        Self(api::Notification::Update {
            patch: patch.clone(),
            commands: commands.into_iter().map(Into::into).collect(),
        })
    }

    pub fn sync(state: State, commands: Vec<Command>) -> Self {
        Self(api::Notification::Sync {
            state: state.into(),
            commands: commands.into_iter().map(Into::into).collect(),
        })
    }

    pub fn error(err: impl std::fmt::Display) -> Self {
        Notification(api::Notification::Err(err.to_string()))
    }
}

impl Handler<Notification> for SocketHandler {
    type Result = ();

    fn handle(&mut self, Notification(notification): Notification, ctx: &mut Self::Context) {
        ctx.text(serde_json::to_string(&notification).unwrap());
    }
}
