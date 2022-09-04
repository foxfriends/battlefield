use crate::components::canvas_2d::Canvas2d;
use crate::components::game_socket_provider::GameSocketProvider;
use crate::pages::game::GamePage;
use uuid::Uuid;
use yew::prelude::*;

#[derive(Clone, Eq, PartialEq, Properties)]
pub struct Props {
    pub id: Uuid,
}

#[function_component(Index)]
pub fn index(props: &Props) -> Html {
    html! {
        <GameSocketProvider url={format!("ws://localhost:8080/ws/{}", props.id)}>
            <Canvas2d>
                <GamePage />
            </Canvas2d>
        </GameSocketProvider>
    }
}
