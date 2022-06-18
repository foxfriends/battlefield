use super::Module;
use crate::graphql::schema::connection::{connection, IteratorConnector};
use crate::graphql::schema::Context;

pub struct ModulesConnector<'a> {
    context: &'a Context,
}

impl<'a> ModulesConnector<'a> {
    pub fn new(context: &'a Context) -> Self {
        Self { context }
    }
}

impl<'a> IteratorConnector for ModulesConnector<'a> {
    type Node = Module<'a>;
    type Iter = Box<dyn Iterator<Item = Module<'a>> + 'a>;
    type IterRev = Box<dyn Iterator<Item = Module<'a>> + 'a>;

    fn len(&self) -> usize {
        self.context.engine.modules().len()
    }

    fn items(&self) -> Self::Iter {
        Box::new(self.context.engine.modules().map(Module))
    }

    fn items_rev(&self) -> Self::IterRev {
        // HACK: since hashmap iterators don't reverse.
        // Maybe there are more appropriate solutions, but this will do for now.
        let all_items = self.context.engine.modules().collect::<Vec<_>>();
        Box::new(all_items.into_iter().rev().map(Module))
    }
}

connection!(impl<'a> for ModulesConnector<'a> as "ModulesConnection");
