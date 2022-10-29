use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
        <div>
            <Link<Route> to={Route::Home}> {"Home"} </Link<Route>> {" "}
            <Link<Route> to={Route::Game}> {"Game"} </Link<Route>> {" "}
            <Link<Route> to={Route::Get}> {"Get"} </Link<Route>> {" "}
        </div>
    }
}
