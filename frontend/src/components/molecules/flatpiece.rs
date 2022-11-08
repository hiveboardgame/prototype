use crate::components::common::piecetype::{self, PieceType};
use crate::components::common::svgpos::SvgPos;
use crate::stores::gamestate::GameStateStore;
use crate::stylesheets::flatpieces::FlatPieceStyle;
use gloo::console::log;
use hive_lib::{piece::Piece, position::Position};
use stylist::{style, yew::styled_component};
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FlatPieceProps {
    pub piece: Piece,
    pub center_offset: (f32, f32),
    pub position: Position,
    pub size: u32,
    pub zoom: u32,
    pub piecetype: PieceType,
}

#[styled_component(FlatPiece)]
pub fn flatpiece(props: &FlatPieceProps) -> Html {
    let color = props.piece.color.to_html_color().to_string().clone();
    let bug = props.piece.bug.as_emoji();
    let bug_size = format!("{}em", props.zoom as f32 * 1.0);
    let svg_pos = SvgPos::new(props.position.0, props.position.1);
    let points = svg_pos.corner_string_with_offset(props.size as f32, props.center_offset);
    let center = svg_pos.center_with_offset(props.size as f32, props.center_offset);
    let transform = format!("translate({},{})", center.0, center.1);

    let (store, state_dispatch) = use_store::<GameStateStore>();

    let onclick_log = {
        let dispatch = state_dispatch.clone();
        let store = store.clone();
        let piece = props.piece.clone();
        let position = props.position.clone();
        match props.piecetype.clone() {
            PieceType::Spawn => Callback::from(move |_| {
                log!("I spawn the piece");
                dispatch.reduce_mut(|store| store.spawn_active_piece());
            }),
            PieceType::Active => Callback::from(move |_| {
                log!("I am the active piece");
            }),
            PieceType::Covered => Callback::from(move |_| {
                log!("You can't click me! I am covered");
            }),
            PieceType::Board => Callback::from(move |_| {
                log!("I am a board piece");
                if store.state.board.queen_played(&store.state.turn_color) {
                    dispatch.reduce_mut(|store| store.show_moves(piece, position));
                }
            }),
            PieceType::Inactive => Callback::from(move |_| {
                log!("I don't do anything");
                if piece.color == store.state.turn_color {
                    dispatch.reduce_mut(|store| store.reset());
                }
            }),
            PieceType::Reserve => Callback::from(move |_| {
                log!("I am a reserve piece"); //, props.piece.clone().to_string());
                dispatch.reduce_mut(|store| store.show_spawns(piece));
            }),
        }
    };

    let stylesheet = style!(
        r#"
            @keyframes blink {
                100%,
                0% {
                    opacity: 0.1;
                }
                60% {
                    opacity: 1.0;
                    }
            }

            @keyframes darkblink {
                100%,
                0% {
                    opacity: 0.1;
                }
                60% {
                    opacity: 0.3;
                    }
            }

            #spawn {
                animation: blink 1.3s infinite;
            }

            #active {
                animation: darkblink 1.3s infinite; 
            }

            #inactive {
                opacity: 0.6;
            }

            #covered {
                opacity: 1.0;
            }
        "#
    )
    .expect("FlatPiece styling failed");

    let mut piecetype = props.piecetype.to_string();
    if let Some(active) = store.active {
        if active == props.piece && piecetype != "spawn" {
            piecetype = "active".to_owned();
        }
    }

    if piecetype == "inactive" {
        return html! {
            <>
            <g class={stylesheet}>
                <g onclick={onclick_log.clone()} fill={color} stroke="grey">
                   <polygon points={points.clone()}></polygon>
                </g>
                <g onclick={onclick_log} {transform}><text text-anchor="middle" dominant-baseline="middle" font-size={bug_size}>{bug}</text></g>
                <g id={piecetype.clone()} fill="grey" stroke="grey">
                   <polygon points={points.clone()}></polygon>
                </g>
            </g>
            </>
        }
    }
    html! {
        <>
        <g class={stylesheet}>
            <g id={piecetype.clone()} onclick={onclick_log.clone()} fill={color.clone()} stroke={color.clone()}>
                <polygon points={points.clone()}></polygon>
            </g>
            <g id={piecetype} onclick={onclick_log} {transform}><text text-anchor="middle" dominant-baseline="middle" font-size={bug_size}>{bug}</text></g>
        </g>
        </>
    }
}
