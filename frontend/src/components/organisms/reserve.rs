use crate::components::molecules::boardpiece::{BoardPiece, Pos};
use hive_lib::{board::Board, bug::Bug, color::Color, piece::Piece};
use web_sys;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ReserveProps {
    pub board: Board,
    pub zoom: u32,
    pub color: Color,
}

#[function_component(Reserve)]
pub fn reserve(props: &ReserveProps) -> Html {
    let reserve = props.board.reserve(props.color);
    let vec = reserve
        .iter()
        .map(|(bug, count)| vec![bug.clone(); *count as usize])
        .flatten()
        .collect::<Vec<Bug>>();
    let len = (vec.len() as f32 / 2.0).round() as i8;
    let pos_pieces = vec.iter().enumerate().map(|(i, bug)| {
        (
            Pos::new((-1 * len as i8 + i as i8), 0),
            Piece::new(bug.clone(), props.color.clone(), None),
        )
    }).collect::<Vec<(Pos, Piece)>>();
    let window = web_sys::window().unwrap();
    let height = window.inner_height().unwrap().as_f64().unwrap();
    let width = window.inner_width().unwrap().as_f64().unwrap();
    let vb = format! {"{} {} {} {}", -0.2*width, -0.05*height, width*0.4, height*0.1};

    html! {
        <svg viewBox={vb}>
        {
            for pos_pieces.iter().map(|(pos, piece)| {
                html_nested! {
                    <BoardPiece piece={*piece} pos={pos.clone()} zoom={1} size={25}/>
                }
            })
        }
        </svg>
    }
}
