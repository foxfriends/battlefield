use super::Context;
use juniper::{EmptyMutation, EmptySubscription, RootNode};

mod connection;
mod query;

use query::Query;

pub type Schema = RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>;

pub fn schema() -> Schema {
    Schema::new(Query, EmptyMutation::new(), EmptySubscription::new())
}
