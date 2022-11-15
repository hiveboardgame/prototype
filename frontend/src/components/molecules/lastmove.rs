use crate::components::common::svgpos::SvgPos;
use hive_lib::position::Position;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LastMoveProps {
    pub pos: Position,
}

#[function_component(LastMove)]
pub fn lastmove(props: &LastMoveProps) -> Html {
    let svg_pos = SvgPos::new(props.pos.0, props.pos.1);
    let center = svg_pos.center();
    let transform = format!("translate({},{})", center.0, center.1);

    let mut filter = "filter: drop-shadow(0.3px 0.3px 0.3px #000)";
    html! {
        <>
            <g {transform} style={filter}>
                <use href="#lastmove" transform="scale(0.56, 0.56) translate(-46.608, -52.083)" />
            </g>
        </>
    }
}
