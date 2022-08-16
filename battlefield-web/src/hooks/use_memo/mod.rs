use yew::{use_effect_with_deps, use_state};

pub fn use_memo<T, Callback, Dependents>(
    callback: Callback,
    deps: Dependents,
) -> impl std::ops::Deref<Target = T>
where
    Callback: Fn(&Dependents) -> T + 'static,
    T: 'static,
    Dependents: PartialEq + 'static,
{
    let state = use_state(|| callback(&deps));
    let setter = state.clone();
    use_effect_with_deps(
        move |deps| {
            setter.set(callback(deps));
            || ()
        },
        deps,
    );
    state
}
