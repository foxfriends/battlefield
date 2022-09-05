use crate::components::context_2d_provider::use_context_2d;
use crate::components::game_state_provider::use_game_state;
use crate::hooks::use_animation_frame::use_animation_frame;
use yew::prelude::*;

#[function_component(GamePage)]
pub fn game_page() -> Html {
    let ctx = use_context_2d();
    let _game_state = use_game_state();

    use_animation_frame(
        move |_, ctx| {
            ctx.clear_rect(
                0.0,
                0.0,
                ctx.canvas().unwrap().width() as f64,
                ctx.canvas().unwrap().height() as f64,
            );
        },
        ctx,
    );

    html! {
        <div />
    }
}
