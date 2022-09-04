use std::cell::Cell;
use std::future::Future;
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use yew::{use_effect_with_deps, use_state};

#[derive(Debug)]
pub struct QueryState<T> {
    pub is_loading: bool,
    pub data: Option<Rc<T>>,
}

impl<T> Clone for QueryState<T> {
    fn clone(&self) -> Self {
        Self {
            is_loading: self.is_loading,
            data: self.data.clone(),
        }
    }
}

impl<T> Default for QueryState<T> {
    fn default() -> Self {
        Self {
            is_loading: false,
            data: None,
        }
    }
}

pub fn use_query<T, F, Callback, Dependents>(callback: Callback, deps: Dependents) -> QueryState<T>
where
    T: 'static,
    F: Future<Output = T> + 'static,
    Callback: Fn(&Dependents) -> F + 'static,
    Dependents: PartialEq + 'static,
{
    let state = use_state(QueryState::default);
    use_effect_with_deps(
        {
            let state = state.clone();
            move |deps| {
                let fresh = Rc::new(Cell::new(true));
                spawn_local({
                    let fresh = fresh.clone();
                    let call = callback(deps);
                    async move {
                        state.set(QueryState {
                            is_loading: true,
                            data: state.data.clone(),
                        });
                        let result = call.await;
                        if fresh.get() {
                            state.set(QueryState {
                                is_loading: false,
                                data: Some(Rc::new(result)),
                            });
                        }
                    }
                });
                move || fresh.set(false)
            }
        },
        deps,
    );
    (*state).clone()
}
