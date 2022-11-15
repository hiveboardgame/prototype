use crate::{components::molecules::stackedpieces::StackedPieces, stores::gamestate::GameStateStore};
use hive_lib::{board::Board, bug::Bug, color::Color, piece::Piece, position::Position};
use web_sys;
use yew::prelude::*;
use yewdux::prelude::*;
use crate::components::common::piecetype::PieceType;
use crate::components::svgs::bugs::Bugs;

#[derive(PartialEq)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Properties, PartialEq)]
pub struct ReserveProps {
    pub board: Board,
    pub color: Color,
    pub orientation: Orientation,
}

#[function_component(Reserve)]
pub fn reserve(props: &ReserveProps) -> Html {
    let reserve = props.board.reserve(&props.color);
    let len = reserve.iter().fold(0, |acc, (_, amount)| acc + amount);
    let (store, _dispatch) = use_store::<GameStateStore>();
    let pos_pieces = Bug::all()
        .iter()
        .filter_map(|bug| match reserve.get(bug) {
            Some(count) if *count > 0 => {
                // TODO: think about moving that to the engine
                let order = match bug {
                    Bug::Ant => Some(4 - count),
                    Bug::Beetle => Some(3 - count),
                    Bug::Grasshopper => Some(4 - count),
                    Bug::Ladybug => None,
                    Bug::Mosquito => None,
                    Bug::Pillbug => None,
                    Bug::Queen => None,
                    Bug::Spider => Some(3 - count),
                };
                Some(vec![Piece::new(bug.clone(), props.color.clone(), order); *count as usize],)
            },
            _ => None,
        }).into_iter()
        .enumerate()
        .map(|(i, pieces)| {
             let piecetype = {
                let mut piecetype = if store.state.turn_color == props.color {
                    PieceType::Reserve
                } else {
                    PieceType::Inactive
                };
                if let Some(piece) = pieces.last() {
                    if piece.bug == Bug::Queen && !store.state.queen_allowed() {
                        piecetype = PieceType::Inactive;
                    }
                }
                if store.state.board.queen_required(store.state.turn, &store.state.turn_color) {
                    if let Some(piece) = pieces.last() {
                        if piece.bug != Bug:: Queen {
                            piecetype = PieceType::Inactive;
                        }
                    }
                }
                piecetype
             };
             // TODO: calculate position from vb size
            match props.orientation {
               Orientation::Horizontal => (Position::new(-1 * len as i8 + i as i8, 0), piecetype, pieces),
               Orientation::Vertical => (Position::new(1, 1*i as i8), piecetype, pieces),
            }
        })
        .collect::<Vec<(Position, PieceType, Vec<Piece>)>>();
    let window = web_sys::window().unwrap();
    let height = window.inner_height().unwrap().as_f64().unwrap();
    let width = window.inner_width().unwrap().as_f64().unwrap();
    let vb = match props.orientation {
                Orientation::Horizontal => format! {"{} {} {} {}", -0.4*width, -0.03*height, width*0.4, height*0.06},
                Orientation::Vertical => format! {"{} {} {} {}", -0.0*width, -0.1 * height, width*0.1, height*0.9},
    };

    html! {
        <svg viewBox={vb}>
        <Bugs />
        { 
            for pos_pieces.iter().map(|(pos, piecetype, pieces)| {
                html_nested! {
                    <StackedPieces pieces={pieces.clone()} position={pos.clone()} piecetype={piecetype.clone()} />
                }
            })
        }
        </svg>
    }
}
