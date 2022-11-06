use crate::components::common::piecetype::PieceType;
use crate::components::common::svgpos::SvgPos;
use gloo::console::log;
use hive_lib::{piece::Piece, position::Position};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FlatPieceProps {
    pub piece: Piece,
    pub center_offset: (f32, f32),
    pub position: Position,
    pub size: u32,
    pub zoom: u32,
    pub piecetype: PieceType,
}

#[function_component(FlatPiece)]
pub fn flatpiece(props: &FlatPieceProps) -> Html {
    let color = props.piece.color.to_html_color().to_string().clone();
    let bug = props.piece.bug.as_emoji();
    let bug_size = format!("{}em", props.zoom as f32 * 1.0);
    let svg_pos = SvgPos::new(props.position.0, props.position.1);
    let points = svg_pos.corner_string_with_offset(props.size as f32, props.center_offset);
    let center = svg_pos.center_with_offset(props.size as f32, props.center_offset);
    let transform = format!("translate({},{})", center.0, center.1);

    let onclick_log = match props.piecetype.clone() {
        PieceType::Covered => Callback::from(move |_| {
            log!("You can't click me! I am covered");
        }),
        PieceType::Board => Callback::from(move |_| {
            log!("I am a board piece");//, props.position.clone().to_string(), props.piece.clone().to_string());
        }),
        PieceType::Reserve => Callback::from(move |_| {
            log!("I am a reserve piece");//, props.piece.clone().to_string());
        }),
    };

    html! {
        <>
        <g onclick={onclick_log.clone()} fill={color} stroke="grey">
           <polygon points={points}></polygon>
        </g>
        <g onclick={onclick_log} {transform}><text text-anchor="middle" dominant-baseline="middle" font-size={bug_size}>{bug}</text></g>
        </>
    }
}
