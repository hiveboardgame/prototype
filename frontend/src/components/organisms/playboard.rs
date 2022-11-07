use crate::components::common::piecetype::PieceType;
use crate::components::molecules::destination::Destination;
use crate::components::molecules::stackedpieces::StackedPieces;
use crate::components::molecules::flatpiece::FlatPiece;
use crate::stores::gamestate::GameStateStore;
use web_sys;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(PlayBoard)]
pub fn playboard() -> Html {
    let window = web_sys::window().unwrap();
    let height = window.inner_height().unwrap().as_f64().unwrap();
    let width = window.inner_width().unwrap().as_f64().unwrap();
    let vb = format! {"{} {} {} {}", -0.2*width, -0.2*height, width*0.4, height*0.4};

    let (store, dispatch) = use_store::<GameStateStore>();

    html! {
        <svg viewBox={vb}>
            {
                for store.state.board.board.iter().map(|(pos, pieces)| {
                    html_nested! {
                        <StackedPieces pieces={pieces.clone()} position={pos.clone()} piecetype={PieceType::Board} zoom={2} size={30}/>
                    }
                })
            }
            {
                for store.target_postitions.iter().map(|pos| {
                    html_nested! {
                        <Destination position={pos.clone()} zoom={2} size={30}/>
                    }
                })
            }
        </svg>
    }
}
