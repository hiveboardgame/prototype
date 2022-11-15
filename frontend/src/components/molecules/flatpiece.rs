use crate::components::common::piecetype::PieceType;
use crate::components::common::svgpos::SvgPos;
use crate::stores::gamestate::GameStateStore;
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
    let svg_pos = SvgPos::new(props.position.0, props.position.1);
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
                log!("I am inactive");
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

    let mut filter = "filter: drop-shadow(0.3px 0.3px 0.3px #000)";
    if piecetype.clone() == "inactive" {
        filter = "filter: sepia(1)";
    }
    html! {
        <>
        <g class={stylesheet}>
            <g id={piecetype.clone()} onclick={onclick_log.clone()} {transform} style={filter}>
                    <use href={format!("#{}", props.piece.color.name())} transform="scale(0.56, 0.56) translate(-45, -50)" />
                    <use href={format!("#{}", props.piece.bug.name())} transform="scale(0.56, 0.56) translate(-50, -45)"/>
            </g>
        </g>
        </>
    }
}
