use crate::components::context_2d_provider::use_context_2d;
use crate::hooks::use_animation_frame::use_animation_frame;
use wasm_bindgen::JsValue;
use yew::prelude::*;

#[function_component(Game)]
pub fn game() -> Html {
    let ctx = use_context_2d();

    use_animation_frame(
        move |_, ctx| {
            ctx.clear_rect(
                0.0,
                0.0,
                ctx.canvas().unwrap().width() as f64,
                ctx.canvas().unwrap().height() as f64,
            );
            ctx.set_fill_style(&JsValue::from_str("black"));
            ctx.set_font("12pt sans-serif");
            ctx.set_text_baseline("top");
            ctx.fill_text("Hello World from Canvas", 0.0, 0.0).unwrap();
        },
        ctx,
    );

    html! {
        <div class="mt-[12pt]">
            {"Hello World from HTML"}
        </div>
    }
}
