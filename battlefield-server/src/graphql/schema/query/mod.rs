use super::connection::{ConnectionResult, Connector, Cursor};
use super::Context;
use juniper::FieldResult;

mod game;
use game::GamesConnector;

mod module;
use module::ModulesConnector;

mod scenario;
use scenario::ScenariosConnector;

pub struct Query;

#[juniper::graphql_object(context = Context)]
impl Query {
    /// Currently running version of Battlefield server
    fn version() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    /// Lists scenarios loaded by this server
    async fn scenarios_connection<'a>(
        &self,
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

    /// Lists modules loaded by this server
    async fn modules_connection<'a>(
        &self,
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
        &self,
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
}
