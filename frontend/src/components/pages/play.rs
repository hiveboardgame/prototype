use crate::components::organisms::playboard::PlayBoard;
use crate::components::organisms::reserve::{Orientation, Reserve};
use crate::stores::gamestate::GameStateStore;
use hive_lib::color::Color;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(Play)]
pub fn play() -> Html {
    let (store, state_dispatch) = use_store::<GameStateStore>();

    html! {
        <div>
            <h1>
                {"Play"}
            </h1>
            <div class="row" style="width: 100%;">
                <div class="collumn" style="float:left; width: 10%">
                    <Reserve board={store.state.board.clone()} orientation={Orientation::Vertical} color={Color::Black} zoom=1/>
                </div>
                <div class="collumn" style="float:left; width: 80%">
                    <PlayBoard />
                </div>
                <div class="collumn" style="float:right; width: 10%">
                    <Reserve board={store.state.board.clone()} orientation={Orientation::Vertical} color={Color::White} zoom=2/>
                </div>
            </div>
            <div style="clear:both"></div>

        </div>
    }
}
