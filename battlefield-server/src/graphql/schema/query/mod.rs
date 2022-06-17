use super::connection::{ConnectionResult, Connector, Cursor};
use super::Context;

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
    fn scenarios_connection<'a>(
        &self,
        context: &'a Context,
        first: Option<i32>,
        after: Option<Cursor>,
        last: Option<i32>,
        before: Option<Cursor>,
    ) -> ConnectionResult<ScenariosConnector<'a>> {
        ScenariosConnector::new(context).get(
            first.map(Into::into),
            after,
            last.map(Into::into),
            before,
        )
    }
}
