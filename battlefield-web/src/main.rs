use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        "Hello World"
    }
}

fn main() {
    let app_root = gloo::utils::document()
        .query_selector("#app")
        .unwrap()
        .expect("Must be run on a page with `#app`");
    yew::start_app_in_element::<App>(app_root);
}
