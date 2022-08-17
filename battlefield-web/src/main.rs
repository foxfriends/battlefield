use yew::prelude::*;

mod components;
mod game;
mod hooks;

use components::canvas_2d::Canvas2d;
use game::Game;

#[function_component(App)]
fn app() -> Html {
    html! {
        <Canvas2d>
            <Game />
        </Canvas2d>
    }
}

fn main() {
    let app_root = gloo::utils::document()
        .query_selector("#app")
        .unwrap()
        .expect("Must be run on a page with `#app`");
    yew::start_app_in_element::<App>(app_root);
}
