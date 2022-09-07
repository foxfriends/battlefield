use gloo::storage::{SessionStorage, Storage};
use serde::{de::DeserializeOwned, Serialize};
use yew::prelude::*;

pub fn use_storage<S: Storage, T: Serialize + DeserializeOwned + PartialEq + 'static>(
    key: &'static str,
) -> UseStateHandle<Option<T>> {
    let state = use_state(|| S::get(key).ok());

    use_effect_with_deps(
        move |state| {
            match &**state {
                Some(value) => {
                    S::set(key, value).ok();
                }
                None => {
                    S::delete(key);
                }
            }
            || {}
        },
        state.clone(),
    );

    state
}

pub fn use_session_storage<T: Serialize + DeserializeOwned + PartialEq + 'static>(
    key: &'static str,
) -> UseStateHandle<Option<T>> {
    use_storage::<SessionStorage, T>(key)
}
