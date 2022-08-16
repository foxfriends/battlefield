use gloo::events::EventListener;
use gloo::utils::window;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::prelude::*;

mod components;
mod game;
mod hooks;

use components::context_2d_provider::Context2dProvider;
use game::Game;
use hooks::use_memo::use_memo;

#[function_component(App)]
fn app() -> Html {
    let canvas = use_node_ref();
    let context = use_memo(
        |canvas| {
            canvas
                .cast::<HtmlCanvasElement>()?
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<CanvasRenderingContext2d>()
                .ok()
        },
        canvas.clone(),
    );

    use_effect_with_deps(
        {
            |context: &Option<CanvasRenderingContext2d>| {
                let context = context.clone();
                if let Some(context) = &context {
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
                    if let Some(context) = &context {
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
        <Context2dProvider value={context.clone()}>
            <div class="relative w-screen h-screen">
                <canvas ref={canvas} class="absolute w-full h-full" />
                <div class="absolute inset-0 text-black">
                    if context.is_some() {
                        <Game />
                    }
                </div>
            </div>
        </Context2dProvider>
    }
}

fn main() {
    let app_root = gloo::utils::document()
        .query_selector("#app")
        .unwrap()
        .expect("Must be run on a page with `#app`");
    yew::start_app_in_element::<App>(app_root);
}
