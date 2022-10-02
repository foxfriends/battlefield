use super::connection::{ConnectionResult, Connector, Cursor};
use super::Context;
use crate::database;
use juniper::FieldResult;

mod game;
pub use game::Game;
use game::GamesConnector;

mod module;
pub use module::Module;
use module::ModulesConnector;

mod scenario;
pub use scenario::Scenario;
use scenario::ScenariosConnector;

mod maps;
pub use maps::Map;
use maps::MapsConnector;

mod module_config;
pub use module_config::{ModuleConfig, ModuleConfigEntry};

pub struct Query;

#[juniper::graphql_object(context = Context)]
impl Query {
    /// Currently running version of Battlefield server
    fn version() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    /// Lists scenarios loaded by this server
    async fn scenarios_connection<'a>(
        &'a self,
        context: &'a Context,
        first: Option<i32>,
        after: Option<Cursor>,
        last: Option<i32>,
        before: Option<Cursor>,
    ) -> FieldResult<ConnectionResult<ScenariosConnector<'a>>> {
        ScenariosConnector::new(context)
            .get(first.map(Into::into), after, last.map(Into::into), before)
            .await
    }

    /// Lists scenarios loaded by this server
    async fn maps_connection<'a>(
        &'a self,
        context: &'a Context,
        first: Option<i32>,
        after: Option<Cursor>,
        last: Option<i32>,
        before: Option<Cursor>,
    ) -> FieldResult<ConnectionResult<MapsConnector<'a>>> {
        MapsConnector::new(context)
            .get(first.map(Into::into), after, last.map(Into::into), before)
            .await
    }

    /// Lists modules loaded by this server
    async fn modules_connection<'a>(
        &'a self,
        context: &'a Context,
        first: Option<i32>,
        after: Option<Cursor>,
        last: Option<i32>,
        before: Option<Cursor>,
    ) -> FieldResult<ConnectionResult<ModulesConnector<'a>>> {
        ModulesConnector::new(context)
            .get(first.map(Into::into), after, last.map(Into::into), before)
            .await
    }

    /// Lists games played on this server
    async fn games_connection<'a>(
        &'a self,
        context: &'a Context,
        first: Option<i32>,
        after: Option<Cursor>,
        last: Option<i32>,
        before: Option<Cursor>,
    ) -> FieldResult<ConnectionResult<GamesConnector<'a>>> {
        GamesConnector::new(context)
            .get(first.map(Into::into), after, last.map(Into::into), before)
            .await
    }

    /// Retrieve a specific game by ID.
    async fn game(&self, context: &Context, id: String) -> FieldResult<Game> {
        Ok(Game(
            database::Game::load(id.parse()?, &context.database).await?,
        ))
    }
}
