use std::cell::Cell;
use yew::{use_effect_with_deps, use_ref, use_state};

pub fn use_memo<T, Callback, Dependents>(
    callback: Callback,
    deps: Dependents,
) -> impl std::ops::Deref<Target = T>
where
    Callback: Fn(&Dependents) -> T + 'static,
    T: 'static,
    Dependents: PartialEq + 'static,
{
    let skip = use_ref(|| Cell::new(true));
    let state = use_state(|| callback(&deps));
    use_effect_with_deps(
        {
            let setter = state.clone();
            move |deps| {
                if !skip.get() {
                    setter.set(callback(deps));
                }
                skip.set(false);
                || ()
            }
        },
        deps,
    );
    state
}
