use crate::hooks::use_memo::use_memo;
use gloo::net::websocket::futures::WebSocket;
use yew::prelude::*;

mod game_socket;

pub use game_socket::GameSocket;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub url: String,
    pub children: Children,
}

#[function_component(GameSocketProvider)]
pub fn game_socket_provider(props: &Props) -> Html {
    let socket = use_memo(
        |url| GameSocket::new(WebSocket::open(url).unwrap()),
        props.url.clone(),
    );

    html! {
        <ContextProvider<GameSocket> context={socket.clone()}>
            {for props.children.iter()}
        </ContextProvider<GameSocket>>
    }
}
