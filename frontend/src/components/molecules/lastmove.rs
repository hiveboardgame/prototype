use crate::components::common::svgpos::SvgPos;
use hive_lib::position::Position;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LastMoveProps {
    pub pos: Position,
    pub size: u32,
    pub zoom: u32,
}

#[function_component(LastMove)]
pub fn lastmove(props: &LastMoveProps) -> Html {
    let svg_pos = SvgPos::new(props.pos.0, props.pos.1);
    let center = svg_pos.center(props.size as f32);
    let transform = format!("translate({},{})", center.0, center.1);

    html! {
        <>
            <g {transform}>
                <use href="#lastmove" transform="scale(0.56, 0.56) translate(-66.5, -13.65) rotate(-30)" />
            </g>
        </>
    }
}
