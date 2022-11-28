use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::pages::{review::Review, home::Home, get::Get, play::Play, ws_echo::WSEcho};

#[derive(Debug, Routable, PartialEq, Clone)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/play")]
    Play,
    #[at("/review")]
    Review,
    #[at("/get")]
    Get,
    #[at("/ws")]
    WS,
}

pub fn switch(route: Route) -> Html {
    return match route {
        Route::Home => html! { <Home /> },
        Route::Review => html! { <Review /> },
        Route::Play => html! { <Play /> },
        Route::Get => html! { <Get /> },
        Route::WS => html! { <WSEcho /> },
    }
}
