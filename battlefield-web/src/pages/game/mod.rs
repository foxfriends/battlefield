use crate::components::context_2d_provider::use_context_2d;
use crate::components::game_state_provider::use_game_state;
use crate::hooks::use_animation_frame::use_animation_frame;
use crate::hooks::use_mirror_ref::use_mirror_ref;
use wasm_bindgen::JsValue;
use yew::prelude::*;

#[function_component(GamePage)]
pub fn game_page() -> Html {
    let ctx = use_context_2d();
    let game_state = use_game_state();
    let game_state_mirror = use_mirror_ref(game_state);

    use_animation_frame(
        move |_, ctx| {
            ctx.clear_rect(
                0.0,
                0.0,
                ctx.canvas().unwrap().width() as f64,
                ctx.canvas().unwrap().height() as f64,
            );
            if let Some(game_state) = game_state_mirror.borrow().as_ref() {
                for (y, row) in game_state.state.map.tiles.iter().enumerate() {
                    for (x, _) in row.iter().enumerate() {
                        ctx.set_fill_style(&JsValue::from("green"));
                        ctx.fill_rect(x as f64 * 34.0 + 1.0, y as f64 * 34.0 + 1.0, 32.0, 32.0);
                    }
                }
            }
        },
        ctx,
    );

    html! {
        <div />
    }
}
