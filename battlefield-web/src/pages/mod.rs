use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::*;

mod game;
mod menu;
mod new_game;

use game::GamePage;
use menu::MenuPage;
use new_game::NewGamePage;

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
        Route::Menu => html! { <MenuPage /> },
        Route::Game { id } => html! { <GamePage id={*id} /> },
        Route::NewGame { scenario } => html! { <NewGamePage scenario={scenario.clone()} /> },
    }
}
