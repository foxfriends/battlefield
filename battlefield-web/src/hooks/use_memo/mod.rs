use std::cell::Cell;
use std::rc::Rc;
use yew::{use_effect_with_deps, use_ref, use_state};

pub fn use_memo<T, Callback, Dependents>(callback: Callback, deps: Dependents) -> Rc<T>
where
    Callback: Fn(&Dependents) -> T + 'static,
    T: 'static,
    Dependents: PartialEq + 'static,
{
    let skip = use_ref(|| Cell::new(true));
    let state = use_state(|| Rc::new(callback(&deps)));
    use_effect_with_deps(
        {
            let setter = state.clone();
            move |deps| {
                if !skip.get() {
                    setter.set(Rc::new(callback(deps)));
                }
                skip.set(false);
                || ()
            }
        },
        deps,
    );
    (*state).clone()
}
