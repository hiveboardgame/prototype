use crate::components::molecules::boardpiece::{BoardPiece, Pos};
use hive_lib::board::Board;
use web_sys;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct BoardProps {
    pub board: Board,
    pub zoom: u32,
}

#[function_component(FBoard)]
pub fn board(props: &BoardProps) -> Html {
    let style = format!("max-width: {}rem;", props.zoom * 100);
    let ((min_x, min_y), (max_x, max_y)) = props.board.mix_max_positions();
    let max = [(min_x - max_x).abs(), (min_y - max_y).abs()].iter().max().unwrap();
    let size = 100 as f32;
    let h = 2.0 * size;
    let w = (3.0 as f32).sqrt() * size;

    let dimension_w = w * max_x as f32;
    let dimension_h = h * max_y as f32;

    let origin_x = -0.5 * dimension_w as f32;
    let origin_y = -0.5 * dimension_h as f32;

    //let vb = format!{"{} {} {} {}", origin_x, origin_y, dimension_w, dimension_h};
    let window = web_sys::window().unwrap();
    let height = window.inner_height().unwrap().as_f64().unwrap();
    let width = window.inner_width().unwrap().as_f64().unwrap();
    let vb = format!{"{} {} {} {}", -0.2*width, -0.2*height, width*0.4, height*0.4};
    gloo::console::log!("Window:", window.inner_width().unwrap());
    gloo::console::log!("Window:", window.inner_height().unwrap());

    html!{
        //<svg viewBox={vb} style={style}>
        <svg viewBox={vb}>
            {
                for props.board.board.iter().map(|(pos, pieces)| {
                    let piece = pieces.last().unwrap().clone();
                    let pos = Pos::new(pos.0, pos.1);
                    html_nested! {
                        <BoardPiece piece={piece} pos={pos} zoom={1} size={25}/>
                    }
                })
            }
        </svg>
    }
}
