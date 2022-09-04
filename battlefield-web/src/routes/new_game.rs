use crate::components::canvas_2d::Canvas2d;
use crate::components::game_socket_provider::GameSocketProvider;
use crate::pages::game::GamePage;
use yew::prelude::*;

#[derive(Clone, Eq, PartialEq, Properties)]
pub struct Props {
    pub scenario: String,
}

#[function_component(Index)]
pub fn index(props: &Props) -> Html {
    html! {
        <GameSocketProvider url={format!("ws://localhost:8080/ws/new/{}", props.scenario)}>
            <Canvas2d>
                <GamePage />
            </Canvas2d>
        </GameSocketProvider>
    }
}
