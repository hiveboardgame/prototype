use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
        <div>
            <Link<Route> to={Route::Home}> {"Home"} </Link<Route>> {" "}
            <Link<Route> to={Route::Review}> {"Review"} </Link<Route>> {" "}
            <Link<Route> to={Route::Play}> {"Play"} </Link<Route>> {" "}
            <Link<Route> to={Route::Get}> {"Get"} </Link<Route>> {" "}
            <Link<Route> to={Route::WS}> {"WS"} </Link<Route>> {" "}
        </div>
    }
}
