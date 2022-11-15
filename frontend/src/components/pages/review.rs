use crate::components::organisms::board::FBoard;
use crate::components::organisms::reserve::{Orientation, Reserve};
use crate::stores::gamestate::GameStateStore;
use gloo::console::log;
use gloo_net::http::Request;
use hive_lib::color::Color;
use hive_lib::history::History;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(Review)]
pub fn review() -> Html {
    let history = use_state(|| History::default());

    let get_game = {
        let history = history.clone();
        Callback::from(move |_| {
            let history = history.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let resp = Request::get("http://127.0.0.1:8081/history")
                    .send()
                    .await
                    .unwrap()
                    .json::<History>()
                    .await
                    .unwrap();
                history.set(resp);
            });
        })
    };

    let (store, state_dispatch) = use_store::<GameStateStore>();

    let next_move = {
        let history = history.clone();
        Callback::from(move |_| {
            let dispatch = state_dispatch.clone();
            dispatch.reduce_mut(|state| {
                let turn = state.state.turn;
                if let Some((piece, position)) = history.moves.get(turn) {
                    log!(
                        "Turn and color:",
                        state.state.turn,
                        state.state.turn_color.to_string()
                    );
                    log!("Playing: ", piece, position);
                    state.state.play_turn_from_notation(&piece, &position);
                }
            });
        })
    };

    html! {
        <div>
            <h1>
                {"Review"}
            </h1>

            <div>
                <button onclick={get_game}>{"Get game for review"}</button>
            </div>
            <div>
                <button onclick={next_move}>{"Next move"}</button>
            </div>
            <div class="row" style="width: 100%;">
                <div class="collumn" style="float:left; width: 50%">
                    <Reserve board={store.state.board.clone()} orientation={Orientation::Horizontal} color={Color::White} zoom=1/>
                </div>
                <div class="collumn" style="float:left; width: 50%">
                    <Reserve board={store.state.board.clone()} orientation={Orientation::Horizontal} color={Color::Black} zoom=1/>
                </div>
            </div>
            <div class="row" style="width: 100%;">
                <div class="collumn" style="float:left; width: 10%" id="history">
                    <ul class="item-list">
                        { history.moves.clone().iter().map(|(piece, pos)| html!{ <li> { format!("{} {}", piece, pos) } </li> }).collect::<Html>() }
                    </ul>
                </div>
                <div class="collumn" style="float:left; width: 90%">
                    <FBoard board={store.state.board.clone()} zoom=1/>
                </div>
            </div>
            <div style="clear:both"></div>
        </div>
    }
}
