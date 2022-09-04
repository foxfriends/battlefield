#![allow(clippy::let_unit_value)]

use yew::prelude::*;
use yew_router::prelude::*;

mod api;
mod components;
mod hooks;
mod pages;
mod routes;

use routes::{switch, Route};

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    let app_root = gloo::utils::document()
        .query_selector("#app")
        .unwrap()
        .expect("Must be run on a page with `#app`");
    yew::start_app_in_element::<App>(app_root);
}
