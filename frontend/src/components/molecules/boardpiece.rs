use std::fmt::format;

use hive_lib::piece::Piece;
use yew::prelude::*;
use gloo::console::log;

#[derive(Properties, PartialEq)]
pub struct PieceProps {
    pub piece: Piece,
    pub pos: Pos,
    pub size: u32,
    pub zoom: u32,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Pos {
    pub pos: (f32, f32),
}

impl Pos {
    pub fn new(x: i8, y: i8) -> Self {
        Self { pos: (x as f32, y as f32) }
    }

    pub fn center(&self, size: f32) -> (f32, f32) {
        let p = self.pos;
        let h = 2.0 * size;
        let w = (3.0 as f32).sqrt() * size as f32;
        return if (p.1 as i32).rem_euclid(2) == 0 {
            // even
            (p.0 * w, p.1 * 0.75 * h)
        } else {
            (0.5 * w + p.0 * w, p.1 * 0.75 * h)
            // odd
        };
    }

    pub fn corners(&self, size: f32) -> Vec<(f32, f32)> {
        let h = 2.0 * size;
        let w = (3.0 as f32).sqrt() * size as f32;
        let c = self.center(size);
        vec![
            (c.0, c.1 + h * 0.5),
            (c.0 - 0.5 * w, c.1 + 0.25 * h),
            (c.0 - 0.5 * w, c.1 - 0.25 * h),
            (c.0, c.1 + -0.5 * h),
            (c.0 + 0.5 * w, c.1 - 0.25 * h),
            (c.0 + 0.5 * w, c.1 + 0.25 * h),
        ]
    }

    pub fn corner_string(&self, size: f32) -> String {
        let c = self.corners(size);
        format!(
            "{},{} {},{} {},{} {},{} {},{} {},{}",
            c[0].0,
            c[0].1,
            c[1].0,
            c[1].1,
            c[2].0,
            c[2].1,
            c[3].0,
            c[3].1,
            c[4].0,
            c[4].1,
            c[5].0,
            c[5].1
        )
    }
}

#[function_component(BoardPiece)]
pub fn boardpiece(props: &PieceProps) -> Html {
    let color = props.piece.color.to_html_color().to_string().clone();
    let bug = props.piece.bug.as_emoji();
    let bug_size = format!("{}em", props.zoom as f32 * 1.5);
    let points = props.pos.corner_string(props.size as f32);
    let center = props.pos.center(props.size as f32);
    let transform = format!("translate({},{})", center.0, center.1);
    log!("Pos:", props.pos.pos.0, props.pos.pos.1);
    log!("Center:", center.0, center.1);
    html! {
        <>
        <g fill={color}>
           <polygon points={points}></polygon>
        </g>
        <g {transform}><text text-anchor="middle" dominant-baseline="middle" font-size={bug_size}>{bug}</text></g>
        </>
    }
}
