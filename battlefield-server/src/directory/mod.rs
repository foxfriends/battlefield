use crate::db::PgPool;
use crate::game::Game;
use actix::prelude::*;
use actix::WeakAddr;
use battlefield_core::Engine;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

mod lookup;
mod new;

pub use lookup::Lookup;
pub use new::New;

#[derive(Clone)]
pub struct Directory {
    games: Arc<Mutex<HashMap<Uuid, WeakAddr<Game>>>>,
    engine: Arc<Engine>,
    db: PgPool,
}

impl Directory {
    pub fn new(db: PgPool, engine: Arc<Engine>) -> Self {
        Self {
            games: Default::default(),
            engine,
            db,
        }
    }
}

impl Actor for Directory {
    type Context = Context<Self>;
}
