use crate::{components::session_gate::use_session, hooks::use_memo::use_memo};
use battlefield_api::websocket::Notification;
use gloo::net::websocket::{futures::WebSocket, Message};
use std::rc::Rc;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
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
    let session = use_session();
    let socket = use_memo(
        |(url, session)| {
            let socket = GameSocket::new(WebSocket::open(url).unwrap());
            spawn_local(socket.send(Message::Text(session.name.clone())));
            socket
        },
        (props.url.clone(), session),
    );

    #[cfg(debug_assertions)]
    use_effect_with_deps(
        |socket| {
            let callback = Rc::new(Box::new(|notification: &Notification| {
                gloo::console::log!(
                    "GameSocket - Notification Received",
                    JsValue::from_serde(notification).unwrap()
                );
            }) as Box<dyn Fn(&Notification)>);
            let subscription = socket.subscribe(callback);
            move || std::mem::drop(subscription)
        },
        socket.clone(),
    );

    html! {
        <ContextProvider<GameSocket> context={(*socket).clone()}>
            {for props.children.iter()}
        </ContextProvider<GameSocket>>
    }
}

pub fn use_game_socket() -> GameSocket {
    use_context::<GameSocket>().unwrap()
}
