use crate::components::common::piecetype::{self, PieceType};
use crate::components::common::svgpos::SvgPos;
use crate::stores::gamestate::GameStateStore;
use crate::components::molecules::flatpiece::FlatPiece;
use gloo::console::log;
use hive_lib::position::Position;
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DestinationProps {
    pub position: Position,
    pub size: u32,
    pub zoom: u32,
}

#[function_component(Destination)]
pub fn destination(props: &DestinationProps) -> Html {
    let svg_pos = SvgPos::new(props.position.0, props.position.1);
    let points = svg_pos.corner_string_with_offset(props.size as f32, (0.0, 0.0));
    let center = svg_pos.center(props.size as f32);
    let transform = format!("translate({},{})", center.0, center.1);

    let (store, state_dispatch) = use_store::<GameStateStore>();

    let onclick_log = {
        let position = props.position;
        let dispatch = state_dispatch.clone();
        let store = store.clone();
        Callback::from(move |_| {
            if let Some(piece) = store.active {
                dispatch.reduce_mut(|store| store.position = Some(position));
            }
        })
    };

    if store.active.is_some() && store.position.is_none() {
        return html! (
            <>
            <g onclick={onclick_log.clone()} stroke="blue" stroke-width="3">
               <polygon points={points}></polygon>
            </g>
            </>
        )
    }
    if store.active.is_some() && store.position.is_some() {
        return html! (
            <>
                <FlatPiece piece={store.active.unwrap()} position={store.position.unwrap()} center_offset={(0.0, 0.0)} piecetype={PieceType::Spawn} zoom={props.zoom.clone()} size={props.size.clone()}/>
            </>
        )
    }
    html!()
}
