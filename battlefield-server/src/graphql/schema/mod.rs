use super::Context;
use juniper::{EmptyMutation, EmptySubscription, RootNode};

mod query;

use query::Query;

pub(super) type Schema =
    RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>;

pub(super) fn schema() -> Schema {
    Schema::new(Query, EmptyMutation::new(), EmptySubscription::new())
}
