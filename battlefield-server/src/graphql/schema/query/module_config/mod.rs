use super::Context;
use crate::graphql::scalars::Json;
use crate::graphql::schema::key_value::{key_value, KeyValue};
use battlefield_core::data;
use std::borrow::Cow;

pub struct ModuleConfig<'a>(pub(super) Cow<'a, data::ModuleConfig>);

impl<'a> ModuleConfig<'a> {
    pub fn from_ref(data: &'a data::ModuleConfig) -> Self {
        Self(Cow::Borrowed(data))
    }
}

pub type ModuleConfigEntry<'a> = KeyValue<ModuleConfig<'a>>;

key_value!(impl for ModuleConfig<'_> as "ModuleConfigEntry");

#[juniper::graphql_object(context = Context)]
impl ModuleConfig<'_> {
    pub fn id(&self) -> String {
        self.0.id().to_string()
    }

    pub fn name(&self) -> &str {
        self.0.name()
    }

    pub fn version(&self) -> &str {
        self.0.version()
    }

    pub fn config(&self) -> Json<serde_json::Value> {
        Json(serde_json::to_value(self.0.config()).unwrap())
    }
}
