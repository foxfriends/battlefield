pub struct KeyValue<V>(pub(super) String, pub(super) V);

macro_rules! key_value {
    (impl for $t:ty as $n:literal) => {
        #[juniper::graphql_object(name = $n, context = $crate::graphql::Context)]
        impl $crate::graphql::schema::key_value::KeyValue<$t> {
            pub fn key(&self) -> &str {
                &self.0
            }

            pub fn value(&self) -> &$t {
                &self.1
            }
        }
    };
}

pub(crate) use key_value;
