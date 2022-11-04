use crate::components::molecules::flatpiece::{FlatPiece, Pos};
use hive_lib::piece::Piece;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct StackedPiecesProps {
    pub pieces: Vec<Piece>,
    pub pos: Pos,
    pub size: u32,
    pub zoom: u32,
}

#[function_component(StackedPieces)]
pub fn stackedpieces(props: &StackedPiecesProps) -> Html {
    html! {
        for props.pieces.iter().enumerate().map(|(i, piece)|{
            let center_offset = (-2.0 * i as f32, -2.0 * i as f32);
            html_nested! {
                <FlatPiece piece={piece.clone()} pos={props.pos.clone()} center_offset={center_offset} zoom={props.zoom.clone()} size={props.size.clone()}/>
            }
        })
    }
}
