use crate::{bug_stack::BugStack, piece::Piece};

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub struct Hex {
    // Where are neighbors
    pub neighbor_nw: bool,
    pub neighbor_w: bool,
    pub neighbor_sw: bool,
    pub neighbor_se: bool,
    pub neighbor_e: bool,
    pub neighbor_ne: bool,

    // Is the move from current tile to direction gated
    pub gated_nw: bool,
    pub gated_w: bool,
    pub gated_sw: bool,
    pub gated_se: bool,
    pub gated_e: bool,
    pub gated_ne: bool,

    // is there negative_space to direction
    pub negative_space_nw: bool,
    pub negative_space_w: bool,
    pub negative_space_sw: bool,
    pub negative_space_se: bool,
    pub negative_space_e: bool,
    pub negative_space_ne: bool,

    pub is_negative_space: bool,

    // what is actually at the current position
    pub bug_stack: BugStack,
}

impl Hex {
    pub fn new() -> Self {
        Self {
            neighbor_nw: false,
            neighbor_w: false,
            neighbor_sw: false,
            neighbor_se: false,
            neighbor_e: false,
            neighbor_ne: false,

            gated_nw: false,
            gated_w: false,
            gated_sw: false,
            gated_se: false,
            gated_e: false,
            gated_ne: false,

            negative_space_nw: false,
            negative_space_w: false,
            negative_space_sw: false,
            negative_space_se: false,
            negative_space_e: false,
            negative_space_ne: false,

            is_negative_space: false,
            bug_stack: BugStack::new(),
        }
    }
}

// use bitfield_struct::bitfield;
//
// #[bitfield(u64)]
// #[derive(PartialEq, Hash, Eq)]
// pub struct Tile {
//     // Neighbors taken 6 bits
//     #[bits(1)]
//     pub neighbor_nw: bool,
//     #[bits(1)]
//     pub neighbor_w: bool,
//     #[bits(1)]
//     pub neighbor_sw: bool,
//     #[bits(1)]
//     pub neighbor_se: bool,
//     #[bits(1)]
//     pub neighbor_e: bool,
//     #[bits(1)]
//     pub neighbor_ne: bool,
//     #[bits(1)]
//     // Total 6
//     // Movement gated 6 bits
//     pub gated_nw: bool,
//     #[bits(1)]
//     pub gated_w: bool,
//     #[bits(1)]
//     pub gated_sw: bool,
//     #[bits(1)]
//     pub gated_se: bool,
//     #[bits(1)]
//     pub gated_e: bool,
//     #[bits(1)]
//     pub gated_ne: bool,
//     // Total 12
//     // is there negative_space to direction
//     #[bits(1)]
//     pub negative_space_nw: bool,
//     #[bits(1)]
//     pub negative_space_w: bool,
//     #[bits(1)]
//     pub negative_space_sw: bool,
//     #[bits(1)]
//     pub negative_space_se: bool,
//     #[bits(1)]
//     pub negative_space_e: bool,
//     #[bits(1)]
//     pub negative_space_ne: bool,
//     // Total 18
//     // bugs 42 bits (7 * 6 bits)
//     #[bits(1)]
//     pub color_0: Color,
//     #[bits(3)]
//     pub bug_0: Bug,
//     #[bits(2)]
//     pub order_0: usize,
//     #[bits(1)]
//     pub color_1: Color,
//     #[bits(3)]
//     pub bug_1: Bug,
//     #[bits(2)]
//     pub order_1: usize,
//     #[bits(1)]
//     pub color_2: Color,
//     #[bits(3)]
//     pub bug_2: Bug,
//     #[bits(2)]
//     pub order_2: usize,
//     #[bits(1)]
//     pub color_3: Color,
//     #[bits(3)]
//     pub bug_3: Bug,
//     #[bits(2)]
//     pub order_3: usize,
//     #[bits(1)]
//     pub color_4: Color,
//     #[bits(3)]
//     pub bug_4: Bug,
//     #[bits(2)]
//     pub order_4: usize,
//     #[bits(1)]
//     pub color_5: Color,
//     #[bits(3)]
//     pub bug_5: Bug,
//     #[bits(2)]
//     pub order_5: usize,
//     #[bits(1)]
//     pub color_6: Color,
//     #[bits(3)]
//     pub bug_6: Bug,
//     #[bits(2)]
//     pub order_6: usize,
//     // Total 60
//     // how many bugs are there
//     #[bits(3)]
//     size: usize,
//     // Total 63
//     #[bits(1)]
//     is_negative_space: bool,
//     // Total 64
// }
