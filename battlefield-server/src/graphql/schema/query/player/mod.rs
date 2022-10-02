use super::Context;
use battlefield_core::data;
use std::borrow::Cow;

pub struct Player<'a>(pub(super) Cow<'a, data::Player>);

impl<'a> Player<'a> {
    pub fn from_ref(data: &'a data::Player) -> Self {
        Self(Cow::Borrowed(data))
    }
}

#[juniper::graphql_object(context = Context)]
impl Player<'_> {
    pub fn id(&self) -> i32 {
        self.0.id() as i32
    }

    pub fn name(&self) -> &str {
        self.0.name()
    }
}
