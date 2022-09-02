use gloo::net::websocket::futures::WebSocket;
use std::rc::Rc;

#[derive(Clone)]
pub struct GameSocket(Rc<WebSocket>);

impl GameSocket {
    pub(super) fn new(socket: WebSocket) -> Self {
        Self(Rc::new(socket))
    }
}

impl PartialEq for GameSocket {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}
