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
    let points = svg_pos.corner_string_with_offset(center_offset);

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

    let id = "destination";

    let stylesheet = style!(
        r#"
            #destination {
                opacity: 0.5;
            }

            #destination:hover {
                opacity: 1;
            }
        "#
    )
    .expect("Destination style failed to parse");

    // this displays the destinations to chose from
    if store.active.is_some() && store.position.is_none() {
        return html! (
            <g class={stylesheet}>
                <g id={id} onclick={onclick.clone()} stroke="#6c71c4" fill="white" fill-opacity="0.0" stroke-width="3">
                    <polygon points={points}></polygon>
                </g>
            </g>
        );
    }

    // this shows the piece at the new destination
    if store.active.is_some() && store.position.is_some() {
        return html! (
            <>
                <FlatPiece piece={store.active.unwrap()} position={store.position.unwrap()} center_offset={center_offset} piecetype={PieceType::Spawn} />
            </>
        );
    }
    html!()
}
