use crate::components::context_2d_provider::use_context_2d;
use crate::components::game_state_provider::use_game_state;
use crate::hooks::use_animation_frame::use_animation_frame;
use crate::hooks::use_mirror_ref::use_mirror_ref;
use yew::prelude::*;

mod render;

#[function_component(GamePage)]
pub fn game_page() -> Html {
    let ctx = use_context_2d();
    let game_state = use_game_state();
    let game_state_mirror = use_mirror_ref(game_state);

    use_animation_frame(
        move |_, ctx| render::render(ctx, game_state_mirror.borrow().as_ref().as_ref()),
        ctx,
    );

    html! {
        <div />
    }
}
