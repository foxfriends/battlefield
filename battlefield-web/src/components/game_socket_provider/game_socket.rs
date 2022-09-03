use futures::stream::SplitSink;
use futures::{SinkExt, StreamExt};
use gloo::net::websocket::futures::WebSocket;
use gloo::net::websocket::{Message, WebSocketError};
use std::cell::RefCell;
use std::future::ready;
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;

type Callbacks = Rc<RefCell<Vec<Rc<dyn Fn(&Message)>>>>;

#[derive(Clone)]
pub struct GameSocket {
    callbacks: Callbacks,
    sender: Rc<RefCell<SplitSink<WebSocket, Message>>>,
}

pub struct Subscription {
    callback: Rc<dyn Fn(&Message)>,
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
                    Ok(message) => {
                        for callback in &*callbacks.borrow() {
                            callback(&message);
                        }
                    }
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

    pub async fn send(&self, message: Message) {
        self.sender.borrow_mut().send(message).await.ok();
    }

    pub fn subscribe(&self, callback: Rc<dyn Fn(&Message)>) -> Subscription {
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
