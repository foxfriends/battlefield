use super::Scenario;
use crate::graphql::schema::connection::{connection, IteratorConnector};
use crate::graphql::schema::Context;

pub struct ScenariosConnector<'a> {
    context: &'a Context,
}

impl<'a> ScenariosConnector<'a> {
    pub fn new(context: &'a Context) -> Self {
        Self { context }
    }
}

impl<'a> IteratorConnector for ScenariosConnector<'a> {
    type Node = Scenario<'a>;
    type Iter = Box<dyn Iterator<Item = Scenario<'a>> + 'a>;
    type IterRev = Box<dyn Iterator<Item = Scenario<'a>> + 'a>;

    fn len(&self) -> usize {
        self.context.engine.scenarios().len()
    }

    fn items(&self) -> Self::Iter {
        Box::new(self.context.engine.scenarios().map(Scenario))
    }

    fn items_rev(&self) -> Self::IterRev {
        Box::new(self.context.engine.scenarios().rev().map(Scenario))
    }
}

connection!(impl<'a> for ScenariosConnector<'a> as "ScenariosConnection");
