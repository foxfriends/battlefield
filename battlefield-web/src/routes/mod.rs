use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::*;

mod game;
mod menu;

#[derive(Clone, Routable, PartialEq, Eq)]
pub enum Route {
    #[at("/")]
    Menu,
    #[at("/:id")]
    Game { id: Uuid },
}

pub fn switch(route: &Route) -> Html {
    match route {
        Route::Menu => html! { <menu::Index /> },
        Route::Game { id } => html! { <game::Index id={*id} /> },
    }
}
