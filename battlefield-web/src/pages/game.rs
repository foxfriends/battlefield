use crate::components::canvas_2d::Canvas2d;
use crate::components::game_socket_provider::GameSocketProvider;
use crate::game::Game;
use uuid::Uuid;
use yew::prelude::*;

#[derive(Clone, Eq, PartialEq, Properties)]
pub struct Props {
    pub id: Uuid,
}

#[function_component(GamePage)]
pub fn game_page(props: &Props) -> Html {
    html! {
        <GameSocketProvider url={format!("ws://localhost:8080/ws/{}", props.id)}>
            <Canvas2d>
                <Game />
            </Canvas2d>
        </GameSocketProvider>
    }
}
