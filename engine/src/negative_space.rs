// use bitfield_struct::bitfield;
// use std::fmt;
//
// use crate::direction::Direction;
//
// #[bitfield(u8)]
// #[derive(PartialEq, Hash, Eq)]
// pub struct NegativeSpace {
//     #[bits(1)]
//     pub nw: bool,
//     #[bits(1)]
//     pub w: bool,
//     #[bits(1)]
//     pub sw: bool,
//     #[bits(1)]
//     pub se: bool,
//     #[bits(1)]
//     pub e: bool,
//     #[bits(1)]
//     pub ne: bool,
//     #[bits(1)]
//     pub gated: bool,
//     /// we need to fill the u8
//     #[bits(1)]
//     _padding: usize,
// }
//
// impl NegativeSpace {
//     pub fn new_from(directions: Vec<Direction>, gated: bool) -> NegativeSpace {
//         let mut negative_space = NegativeSpace::new().with_gated(gated);
//         for direction in directions.iter() {
//             match direction {
//                 Direction::NW => negative_space.set_nw(true),
//                 Direction::NE => negative_space.set_ne(true),
//                 Direction::SE => negative_space.set_se(true),
//                 Direction::E => negative_space.set_e(true),
//                 Direction::W => negative_space.set_w(true),
//                 Direction::SW => negative_space.set_sw(true),
//             };
//         }
//         negative_space
//     }
// }
//
// impl fmt::Display for NegativeSpace {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(
//             f,
//             "Gated: {} NW: {} W: {} SW: {} SE: {} E: {} NE: {}",
//             self.gated(),
//             self.nw(),
//             self.w(),
//             self.sw(),
//             self.se(),
//             self.e(),
//             self.ne()
//         )
//     }
// }
