use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::*;

mod game;
mod menu;
mod new_game;

#[derive(Clone, Routable, PartialEq, Eq)]
pub enum Route {
    #[at("/")]
    Menu,
    #[at("/:id")]
    Game { id: Uuid },
    #[at("/new/:scenario")]
    NewGame { scenario: String },
}

pub fn switch(route: &Route) -> Html {
    match route {
        Route::Menu => html! { <menu::Index /> },
        Route::Game { id } => html! { <game::Index id={*id} /> },
        Route::NewGame { scenario } => html! { <new_game::Index scenario={scenario.clone()} /> },
    }
}
