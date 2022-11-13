use crate::components::common::piecetype::PieceType;
use crate::components::common::svgpos::SvgPos;
use crate::components::molecules::flatpiece::FlatPiece;
use crate::stores::gamestate::GameStateStore;
use gloo::console::log;
use hive_lib::piece::Piece;
use hive_lib::position::Position;
use stylist::{style, yew::styled_component};
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LastMoveProps {
    pub pos: Position,
    pub size: u32,
    pub zoom: u32,
}

#[styled_component(LastMove)]
pub fn lastmove(props: &LastMoveProps) -> Html {
    let center_offset = (0.0, 0.0);

    let svg_pos = SvgPos::new(props.pos.0, props.pos.1);
    let points = svg_pos.corner_string_with_offset(props.size as f32, center_offset);
    let center = svg_pos.center(props.size as f32);
    let ransform = format!("translate({},{})", center.0, center.1);

    html! {
        <>
            <g stroke="#b58900" fill="white" fill-opacity="0.0" stroke-width="3">
                <polygon points={points}></polygon>
            </g>
        </>
    }
}
