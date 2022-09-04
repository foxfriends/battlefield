use super::Context;
use juniper::{EmptySubscription, RootNode};

mod connection;
mod mutation;
mod query;

use mutation::Mutation;
use query::Query;

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub fn schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::new())
}
