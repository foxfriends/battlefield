use super::game_socket_provider::use_game_socket;
use crate::components::game_socket_provider::GameSocket;
use battlefield_api::websocket::Notification;
use gloo::utils::format::JsValueSerdeExt;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use yew::prelude::*;

mod game_state;

pub use game_state::GameState;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub children: Children,
}

#[function_component(GameStateProvider)]
pub fn game_state_provider(props: &Props) -> Html {
    let socket = use_game_socket();
    let game_state = use_state(|| None);

    use_effect_with_deps(
        {
            let game_state = game_state.clone();
            move |socket: &GameSocket| {
                let callback =
                    Rc::new(
                        Box::new(move |notification: &Notification| match notification {
                            Notification::Init {
                                commands,
                                state,
                                scenario,
                                ..
                            } => game_state.set(Some(GameState::new(
                                commands.clone(),
                                state.clone(),
                                scenario.clone(),
                            ))),
                            Notification::Sync { state, commands } => game_state.set(Some(
                                game_state
                                    .as_ref()
                                    .unwrap()
                                    .sync(commands.clone(), state.clone()),
                            )),
                            Notification::Update { patch, commands } => {
                                let new_state =
                                    game_state.as_ref().unwrap().update(commands.clone(), patch);
                                match new_state {
                                    Ok(new_state) => game_state.set(Some(new_state)),
                                    Err(error) => {
                                        gloo::console::log!(format!(
                                            "Failed to update state from patch: {}",
                                            error
                                        ));
                                    }
                                }
                            }
                            _ => {}
                        }) as Box<dyn Fn(&Notification)>,
                    );
                let subscription = socket.subscribe(callback);
                move || std::mem::drop(subscription)
            }
        },
        socket,
    );

    #[cfg(debug_assertions)]
    use_effect_with_deps(
        |state| {
            gloo::console::log!(
                "GameState",
                <JsValue as JsValueSerdeExt>::from_serde(state).unwrap()
            );
            || ()
        },
        (*game_state).clone(),
    );

    html! {
        <ContextProvider<Option<GameState>> context={(*game_state).clone()}>
            {for props.children.iter()}
        </ContextProvider<Option<GameState>>>
    }
}

pub fn use_game_state() -> Option<GameState> {
    use_context::<Option<GameState>>().unwrap()
}
