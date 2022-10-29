use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::pages::{game::Game, home::Home, get::Get};

#[derive(Debug, Routable, PartialEq, Clone)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/game")]
    Game,
    #[at("/get")]
    Get,
}

pub fn switch(route: &Route) -> Html {
    return match route {
        Route::Home => html! { <Home /> },
        Route::Game => html! { <Game /> },
        Route::Get => html! { <Get /> },
    }
}
