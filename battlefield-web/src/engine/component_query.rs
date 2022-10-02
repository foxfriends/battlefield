use battlefield_api::Entity;
use gloo::utils::format::JsValueSerdeExt;
use wasm_bindgen::JsValue;

use super::Component;

pub trait ComponentQuery {
    type Data;

    fn extract(entity: &Entity) -> Option<Self::Data>;
}

impl<T> ComponentQuery for T
where
    T: Component,
{
    type Data = T::Data;

    fn extract(entity: &Entity) -> Option<Self::Data> {
        let component = entity.components.get(Self::NAME)?;
        serde_json::from_value(component.clone())
            .map_err(|error| {
                gloo::console::log!(
                    format!(
                        "ComponentTypeMismatch: Attempted to access {} as {}: {:?}",
                        Self::NAME,
                        std::any::type_name::<Self::Data>(),
                        error,
                    ),
                    <JsValue as JsValueSerdeExt>::from_serde(&component).unwrap()
                );
            })
            .ok()?
    }
}
