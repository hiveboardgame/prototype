use crate::components::molecules::flatpiece::Pos;
use crate::components::molecules::stackedpieces::StackedPieces;
use hive_lib::{board::Board, bug::Bug, color::Color, piece::Piece};
use web_sys;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ReserveProps {
    pub board: Board,
    pub zoom: u32,
    pub color: Color,
}

#[function_component(Reserve)]
pub fn reserve(props: &ReserveProps) -> Html {
    let reserve = props.board.reserve(props.color);
    let len = reserve.iter().fold(0, |acc, (_, amount)| acc + amount);
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
        .map(|(i, pieces)| 
             ( Pos::new(-1 * len as i8 + i as i8, 0), pieces)
        )
        .collect::<Vec<(Pos, Vec<Piece>)>>();
    let window = web_sys::window().unwrap();
    let height = window.inner_height().unwrap().as_f64().unwrap();
    let width = window.inner_width().unwrap().as_f64().unwrap();
    let vb = format! {"{} {} {} {}", -0.4*width, -0.03*height, width*0.4, height*0.06};

    html! {
        <svg viewBox={vb}>
        { 
            for pos_pieces.iter().map(|(pos, pieces)| {
                html_nested! {
                    <StackedPieces pieces={pieces.clone()} pos={pos.clone()} zoom={1} size={20}/>
                }
            })
        }
        </svg>
    }
}
