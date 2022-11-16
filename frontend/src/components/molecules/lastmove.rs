use crate::components::common::svgpos::SvgPos;
use hive_lib::position::Position;
use yew::prelude::*;

#[derive(PartialEq, Eq)]
pub enum MoveType {
    From,
    To,
}

#[derive(Properties, PartialEq)]
pub struct LastMoveProps {
    pub pos: Position,
    pub m: MoveType,
}

#[function_component(LastMove)]
pub fn lastmove(props: &LastMoveProps) -> Html {
    let svg_pos = SvgPos::new(props.pos.0, props.pos.1);
    let center = svg_pos.center();
    let transform = format!("translate({},{})", center.0, center.1);

    let filter = "filter: drop-shadow(0.3px 0.3px 0.3px #000)";
    html! {
        <>
            <g {transform}>
            if props.m == MoveType::To {
                <use style="color:#d33682" href="#lastmove" transform="scale(0.56, 0.56) translate(-46.608, -52.083)" />
            }
            if props.m == MoveType::From {
                <use style="color:#d33682" href="#destination" transform="scale(0.56, 0.56) translate(-46.608, -52.083)" />
            }
            </g>
        </>
    }
}
