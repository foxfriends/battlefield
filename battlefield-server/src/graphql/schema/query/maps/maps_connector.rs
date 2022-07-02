use super::Map;
use crate::graphql::schema::connection::{connection, IteratorConnector};
use crate::graphql::schema::Context;

pub struct MapsConnector<'a> {
    context: &'a Context,
}

impl<'a> MapsConnector<'a> {
    pub fn new(context: &'a Context) -> Self {
        Self { context }
    }
}

impl<'a> IteratorConnector for MapsConnector<'a> {
    type Node = Map<'a>;
    type Iter = Box<dyn Iterator<Item = Map<'a>> + 'a>;
    type IterRev = Box<dyn Iterator<Item = Map<'a>> + 'a>;

    fn len(&self) -> usize {
        self.context.engine.maps().len()
    }

    fn items(&self) -> Self::Iter {
        Box::new(self.context.engine.maps().map(Map))
    }

    fn items_rev(&self) -> Self::IterRev {
        Box::new(self.context.engine.maps().rev().map(Map))
    }
}

connection!(impl<'a> for MapsConnector<'a> as "MapsConnection");
