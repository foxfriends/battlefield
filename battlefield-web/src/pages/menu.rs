use crate::pages::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(MenuPage)]
pub fn menu_page() -> Html {
    let scenarios = vec!["Field"];

    html! {
        <div>
            {scenarios.into_iter().map(|scenario| {
                html! {
                    <Link<Route> to={Route::NewGame { scenario: scenario.to_owned() }}>
                        {scenario}
                    </Link<Route>>
                }
            }).collect::<Html>()}
        </div>
    }
}
