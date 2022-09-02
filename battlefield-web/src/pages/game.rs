use crate::components::canvas_2d::Canvas2d;
use crate::game::Game;
use uuid::Uuid;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub id: Uuid,
}

#[function_component(GamePage)]
pub fn game_page(props: &Props) -> Html {
    html! {
        <Canvas2d>
            <Game />
        </Canvas2d>
    }
}
