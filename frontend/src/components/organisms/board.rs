use crate::components::molecules::stackedpieces::StackedPieces;
use crate::components::common::piecetype::PieceType;
use hive_lib::board::Board;
use web_sys;
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Properties, PartialEq)]
pub struct BoardProps {
    pub board: Board,
    pub zoom: u32,
}

#[function_component(FBoard)]
pub fn board(props: &BoardProps) -> Html {
    let window = web_sys::window().unwrap();
    let height = window.inner_height().unwrap().as_f64().unwrap();
    let width = window.inner_width().unwrap().as_f64().unwrap();
    let vb = format! {"{} {} {} {}", -0.2*width, -0.2*height, width*0.4, height*0.4};

    html! {
        <svg viewBox={vb}>
            {
                for props.board.board.iter().map(|(pos, pieces)| {
                    html_nested! {
                        <StackedPieces pieces={pieces.clone()} position={pos.clone()} piecetype={PieceType::Covered} zoom={1} size={25}/>
                    }
                })
            }
        </svg>
    }
}
