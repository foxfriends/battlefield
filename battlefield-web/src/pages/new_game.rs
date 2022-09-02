use crate::components::canvas_2d::Canvas2d;
use crate::game::Game;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub scenario: String,
}

#[function_component(NewGamePage)]
pub fn new_game_page(props: &Props) -> Html {
    html! {
        <Canvas2d>
            <Game />
        </Canvas2d>
    }
}
