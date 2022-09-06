mod component;
mod component_query;

pub use component::*;
pub use component_query::*;

macro_rules! component {
    ($id:ident, $name:literal, $data:ty) => {
        pub struct $id;

        impl crate::engine::Component for $id {
            const NAME: &'static str = $name;
            type Data = $data;
        }
    };
}

pub mod components {
    pub mod core {
        component!(Position, "core.position", (usize, usize));
    }
}
