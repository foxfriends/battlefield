use super::Context;

pub struct Query;

#[juniper::graphql_object(context = Context)]
impl Query {
    fn version() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }
}
