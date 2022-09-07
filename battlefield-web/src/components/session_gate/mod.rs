use crate::hooks::use_storage::use_session_storage;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::html::onchange;
use yew::prelude::*;

#[derive(Clone, PartialEq, Eq)]
pub struct Session {
    pub name: String,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub children: Children,
}

#[function_component(SessionGate)]
pub fn session_gate(props: &Props) -> Html {
    let name = use_session_storage::<String>("username");
    let submitted = use_state(|| name.is_some());

    let onchange = Callback::from({
        let name = name.clone();
        move |event: onchange::Event| {
            name.set(Some(
                event
                    .target()
                    .unwrap()
                    .dyn_into::<HtmlInputElement>()
                    .unwrap()
                    .value(),
            ))
        }
    });

    let onsubmit = Callback::from({
        let submitted = submitted.clone();
        let name = name.clone();
        move |_| {
            if name.is_some() && !name.as_ref().unwrap().is_empty() {
                submitted.set(true)
            }
        }
    });

    html! {
        if *submitted {
            <ContextProvider<Session> context={Session { name: (*name).clone().unwrap() }}>
                {for props.children.iter()}
            </ContextProvider<Session>>
        } else {
            <div class="h-full flex flex-col gap-8 items-center justify-center">
                <h1 class="text-2xl font-bold">{"Battlefield: Sign In"}</h1>
                <form onsubmit={onsubmit} class="flex flex-col items-center gap-4">
                    <input
                        type="text"
                        name="Name"
                        placeholder="Enter your name"
                        value={(*name).clone()}
                        onchange={onchange}
                        class="border-b border-black/10 p-2"
                    />
                    <button class="border border-black/10 uppercase text-xs p-1">{"Submit"}</button>
                </form>
            </div>
        }
    }
}
