use gloo::console::log;
use hive_lib::piece::Piece;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FlatPieceProps {
    pub piece: Piece,
    pub center_offset: (f32, f32),
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
        Self {
            pos: (x as f32, y as f32),
        }
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

    pub fn center_with_offset(&self, size: f32, center_offset: (f32, f32)) -> (f32, f32) {
        let center = self.center(size);
        (center.0 + center_offset.0, center.1 + center_offset.1)
    }

    pub fn corners_with_offset(&self, size: f32, center_offset: (f32, f32)) -> Vec<(f32, f32)> {
        let h = 2.0 * size;
        let w = (3.0 as f32).sqrt() * size as f32;
        let c = self.center_with_offset(size, center_offset);
        vec![
            (c.0, c.1 + h * 0.5),
            (c.0 - 0.5 * w, c.1 + 0.25 * h),
            (c.0 - 0.5 * w, c.1 - 0.25 * h),
            (c.0, c.1 + -0.5 * h),
            (c.0 + 0.5 * w, c.1 - 0.25 * h),
            (c.0 + 0.5 * w, c.1 + 0.25 * h),
        ]
    }

    pub fn corner_string_with_offset(&self, size: f32, center_offset: (f32, f32)) -> String {
        let c = self.corners_with_offset(size, center_offset);
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

#[function_component(FlatPiece)]
pub fn flatpiece(props: &FlatPieceProps) -> Html {
    let color = props.piece.color.to_html_color().to_string().clone();
    let bug = props.piece.bug.as_emoji();
    let bug_size = format!("{}em", props.zoom as f32 * 1.0);
    let points = props
        .pos
        .corner_string_with_offset(props.size as f32, props.center_offset);
    let center = props
        .pos
        .center_with_offset(props.size as f32, props.center_offset);
    let transform = format!("translate({},{})", center.0, center.1);

    let onclick_log = {
        Callback::from(move |_| {
            log!("I am a bug");
        })
    };

    html! {
        <>
        <g onclick={onclick_log.clone()} fill={color} stroke="grey">
           <polygon points={points}></polygon>
        </g>
        <g onclick={onclick_log} {transform}><text text-anchor="middle" dominant-baseline="middle" font-size={bug_size}>{bug}</text></g>
        </>
    }
}
