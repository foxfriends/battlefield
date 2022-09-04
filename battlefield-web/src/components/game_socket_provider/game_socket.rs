use battlefield_api::websocket::Notification;
use futures::stream::SplitSink;
use futures::{SinkExt, StreamExt};
use gloo::net::websocket::futures::WebSocket;
use gloo::net::websocket::{Message, WebSocketError};
use std::cell::RefCell;
use std::future::ready;
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;

type Callback = Rc<Box<dyn Fn(&Notification)>>;
type Callbacks = Rc<RefCell<Vec<Callback>>>;

#[derive(Clone)]
pub struct GameSocket {
    callbacks: Callbacks,
    sender: Rc<RefCell<SplitSink<WebSocket, Message>>>,
}

pub struct Subscription {
    callback: Callback,
    callbacks: Callbacks,
}

impl Drop for Subscription {
    fn drop(&mut self) {
        self.callbacks
            .borrow_mut()
            .retain(|el| !Rc::ptr_eq(el, &self.callback))
    }
}

impl GameSocket {
    pub(super) fn new(socket: WebSocket) -> Self {
        let (send, recv) = socket.split();
        let sender = Rc::new(RefCell::new(send));
        let callbacks = Callbacks::default();

        spawn_local({
            let callbacks = callbacks.clone();
            recv.for_each(move |message| {
                match message {
                    Ok(Message::Text(json)) => match serde_json::from_str(&json) {
                        Ok(notification) => {
                            for callback in &*callbacks.borrow() {
                                callback(&notification);
                            }
                        }
                        Err(error) => {
                            gloo::console::error!(format!("{:?}", error));
                        }
                    },
                    Ok(Message::Bytes(..)) => {}
                    Err(WebSocketError::ConnectionClose(..)) => {}
                    Err(error) => {
                        gloo::console::error!(format!("{:?}", error));
                    }
                }
                ready(())
            })
        });

        Self { callbacks, sender }
    }

    #[allow(dead_code, clippy::await_holding_refcell_ref)]
    pub async fn send(&self, message: Message) {
        let mut sender = self.sender.borrow_mut();
        sender.send(message).await.ok();
    }

    pub fn subscribe(&self, callback: Callback) -> Subscription {
        let callbacks = self.callbacks.clone();
        callbacks.borrow_mut().push(callback.clone());
        Subscription {
            callback,
            callbacks,
        }
    }
}

impl PartialEq for GameSocket {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.sender, &other.sender)
    }
}
