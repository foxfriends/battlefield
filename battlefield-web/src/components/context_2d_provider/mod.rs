use crate::hooks::use_memo::use_memo;
use gloo::{events::EventListener, utils::window};
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub canvas: Option<HtmlCanvasElement>,
    pub children: Children,
}

#[function_component(Context2dProvider)]
pub fn context_2d_provider(props: &Props) -> Html {
    let context = use_memo(
        |canvas| {
            canvas
                .as_ref()?
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<CanvasRenderingContext2d>()
                .ok()
        },
        props.canvas.clone(),
    );

    use_effect_with_deps(
        {
            |context: &Rc<Option<CanvasRenderingContext2d>>| {
                let context = context.clone();
                if let Some(context) = context.as_ref() {
                    let canvas = context.canvas().unwrap();
                    let ratio = window().device_pixel_ratio();
                    canvas.set_width(
                        (window().inner_width().unwrap().as_f64().unwrap() * ratio) as u32,
                    );
                    canvas.set_height(
                        (window().inner_height().unwrap().as_f64().unwrap() * ratio) as u32,
                    );
                    context.scale(ratio, ratio).unwrap();
                }
                let listener = EventListener::new(&window(), "resize", move |_event| {
                    if let Some(context) = context.as_ref() {
                        let ratio = window().device_pixel_ratio();
                        let canvas = context.canvas().unwrap();
                        canvas.set_width(
                            (window().inner_width().unwrap().as_f64().unwrap() * ratio) as u32,
                        );
                        canvas.set_height(
                            (window().inner_height().unwrap().as_f64().unwrap() * ratio) as u32,
                        );
                        context.scale(ratio, ratio).unwrap();
                    }
                });
                move || std::mem::drop(listener)
            }
        },
        context.clone(),
    );

    html! {
        <ContextProvider<Option<CanvasRenderingContext2d>> context={(*context).clone()}>
            if context.is_some() {
                {for props.children.iter()}
            }
        </ContextProvider<Option<CanvasRenderingContext2d>>>
    }
}

pub fn use_context_2d() -> CanvasRenderingContext2d {
    use_context::<Option<CanvasRenderingContext2d>>()
        .unwrap()
        .unwrap()
}
