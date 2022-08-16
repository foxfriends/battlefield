use gloo::utils::window;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast};
use yew::use_effect_with_deps;

pub fn use_animation_frame<F, Dependents>(animation: F, deps: Dependents)
where
    F: Clone + Fn(f64, &Dependents) + 'static,
    Dependents: PartialEq + 'static,
{
    use_effect_with_deps(
        move |deps| {
            struct Callback {
                f: Box<dyn Fn(Rc<Callback>)>,
            }

            let animation_frame: Rc<Cell<Option<i32>>> = Rc::default();
            let closure: Rc<RefCell<Option<Closure<_>>>> = Rc::default();

            let callback = Rc::new(Callback {
                f: {
                    let animation_frame = animation_frame.clone();
                    let deps = deps.clone();
                    Box::new(move |callback: Rc<Callback>| {
                        *closure.borrow_mut() = Some(Closure::once({
                            let deps = deps.clone();
                            let animation = animation.clone();
                            move |timestamp: f64| {
                                animation(timestamp, &deps);
                                (callback.f)(callback.clone());
                            }
                        }));
                        animation_frame.set(Some(
                            window()
                                .request_animation_frame(
                                    closure.borrow().as_ref().unwrap().as_ref().unchecked_ref(),
                                )
                                .unwrap(),
                        ));
                    })
                },
            });

            (callback.f)(callback.clone());

            move || {
                if let Some(frame) = animation_frame.get() {
                    window().cancel_animation_frame(frame).unwrap();
                }
                std::mem::drop(callback);
            }
        },
        Rc::new(deps),
    );
}
