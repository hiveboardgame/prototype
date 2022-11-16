use crate::components::common::piecetype::PieceType;
use crate::components::common::svgpos::SvgPos;
use crate::components::molecules::flatpiece::FlatPiece;
use crate::stores::gamestate::GameStateStore;
use gloo::console::log;
use hive_lib::position::Position;
use stylist::{style, yew::styled_component};
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DestinationProps {
    pub position: Position,
}

#[styled_component(Destination)]
pub fn destination(props: &DestinationProps) -> Html {
    let (store, state_dispatch) = use_store::<GameStateStore>();
    let svg_pos = SvgPos::new(props.position.0, props.position.1);
    let mut center_offset = (0.0, 0.0);
    if let Some(pieces) = store.state.board.board.get(&props.position.clone()) {
        center_offset = SvgPos::center_offset(pieces.len());
    }
    let center = svg_pos.center_with_offset(center_offset);
    let transform = format!("translate({},{})", center.0, center.1);

    let onclick = {
        let position = props.position;
        let dispatch = state_dispatch.clone();
        let store = store.clone();
        Callback::from(move |_| {
            log!("I am a destination");
            if store.active.is_some() {
                dispatch.reduce_mut(|store| store.position = Some(position));
            }
        })
    };

    let stylesheet = style!(
        r#"
            .destination {
                color: #6c71c4;
            }

            .destination:hover {
                color: #268bd2;
            }
        "#
    )
    .expect("Destination style failed to parse");

    // this displays the destinations to chose from
    if store.active.is_some() && store.position.is_none() {
        return html! (
            <g {transform} class={stylesheet}>
                <use class="destination"  onclick={onclick} href="#destination" transform="scale(0.56, 0.56) translate(-46.608, -52.083)" />
            </g>
        );
    }
    html!()
}
