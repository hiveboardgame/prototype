//use crate::bug_stack::BugStack;
//use crate::position::Position;
//
//pub struct SpiralArray(Vec<BugStack>);
//impl SpiralArray {
//    pub fn new() -> Self {
//        // For 28 rings, there will be 2437 hexes.
//        SpiralArray(vec![BugStack::new(); 2437])
//    }
//
//    pub fn get(&self, position: Position) -> Option<&BugStack> {
//        self.0.get(Self::position(position))
//    }
//
//    pub fn get_mut(&mut self, position: Position) -> Option<&mut BugStack> {
//        self.0.get_mut(Self::position(position))
//    }
//
//    pub fn insert(&mut self, position: Position, bug_stack: BugStack) {
//        self.0.insert(Self::position(position), bug_stack)
//    }
//
//    fn position(position: Position) -> usize {
//        if position.x == 0 && position.y == 0 {
//            return 0;
//        }
//        let (quadrant_position, index) = Self::quadrant_start(position);
//        index
//            + std::cmp::max(
//                (position.x.abs() - quadrant_position.x.abs()).abs(),
//                (position.y.abs() - quadrant_position.y.abs()).abs(),
//            ) as usize
//    }
//
//    fn quadrant_start(position: Position) -> (Position, usize) {
//        let quadrant = Self::get_quardrant(position);
//        let ring = Self::get_ring(quadrant, position);
//        let index = 1 + 3 * (ring) * (ring - 1) + (quadrant as usize - 1) * ring;
//        let quadrant_position = Self::get_quadrant_position(ring, quadrant);
//        (quadrant_position, index)
//    }
//
//    fn get_quadrant_position(ring: usize, quadrant: Quadrant) -> Position {
//        match quadrant {
//            // x-axis
//            Quadrant::E => Position {
//                x: ring as i8,
//                y: 0i8,
//            },
//            Quadrant::W => Position {
//                x: -1 * ring as i8,
//                y: 0i8,
//            },
//            // y-axis
//            Quadrant::NW => Position {
//                x: 0i8,
//                y: -1 * ring as i8,
//            },
//            Quadrant::SE => Position {
//                x: 0i8,
//                y: ring as i8,
//            },
//            //z-axis
//            Quadrant::NE => Position {
//                x: ring as i8,
//                y: -1 * ring as i8,
//            },
//            Quadrant::SW => Position {
//                x: -1 * ring as i8,
//                y: ring as i8,
//            },
//        }
//    }
//
//    fn get_quardrant(position: Position) -> Quadrant {
//        match (position.x, position.y) {
//            // x axis
//            (0, y) if y < 0 => Quadrant::NW,
//            (0, y) if y > 0 => Quadrant::SE,
//            // y axis
//            (x, 0) if x > 0 => Quadrant::E,
//            (x, 0) if x < 0 => Quadrant::W,
//            // z axis
//            (x, y) if -1 * x == y && x > 0 => Quadrant::NE,
//            (x, y) if -1 * x == y && x < 0 => Quadrant::SW,
//            // SE
//            (x, y) if x > 0 && y > 0 => Quadrant::SE,
//            // NW
//            (x, y) if x < 0 && y < 0 => Quadrant::NW,
//            // NE
//            (x, y) if x > 0 && y < 0 && x.abs() < y.abs() => Quadrant::NE,
//            // E
//            (x, y) if x > 0 && y < 0 && x.abs() > y.abs() => Quadrant::E,
//            // SW
//            (x, y) if x < 0 && y > 0 && x.abs() < y.abs() => Quadrant::SW,
//            // W
//            (x, y) if x < 0 && y > 0 && x.abs() > y.abs() => Quadrant::W,
//            (_, _) => unreachable!(),
//        }
//    }
//
//    fn get_ring(quadrant: Quadrant, position: Position) -> usize {
//        (match quadrant {
//            Quadrant::SE | Quadrant::NW => position.x.abs() + position.y.abs(),
//            _ => std::cmp::max(position.x.abs(), position.y.abs()),
//        }) as usize
//    }
//}
//
//#[derive(Clone, Copy, PartialEq, Eq, Debug)]
//enum Quadrant {
//    E = 1,
//    NE = 2,
//    NW = 3,
//    W = 4,
//    SW = 5,
//    SE = 6,
//}
//
//#[cfg(test)]
//mod tests {
//    use super::*;
//    use crate::{position::Position, bug::Bug, bug::Kind, color::Color};
//
//    #[test]
//    fn test_new_insert_get() {
//        let mut arr = SpiralArray::new();
//        let position = Position { x: 0, y: 0 };
//        let mut bug_stack = BugStack::new();
//        let bug = Bug::new().with_kind(Kind::Ladybug).with_color(Color::White);
//        bug_stack.push_bug(bug);
//        assert!(arr.get(position).is_some());
//        arr.insert(position, bug_stack);
//        assert_eq!(*arr.get(position).unwrap(), bug_stack);
//        assert_eq!(*arr.get(position).unwrap(), bug_stack);
//        assert_eq!(arr.get_mut(position).unwrap().pop_bug(), bug_stack.pop_bug());
//    }
//
//    #[test]
//    fn test_get_quardrant() {
//        // NE
//        assert_eq!(
//            SpiralArray::get_quardrant(Position { x: 1, y: -1 }),
//            Quadrant::NE
//        );
//        assert_eq!(
//            SpiralArray::get_quardrant(Position { x: 1, y: -2 }),
//            Quadrant::NE
//        );
//        assert_eq!(
//            SpiralArray::get_quardrant(Position { x: 2, y: -3 }),
//            Quadrant::NE
//        );
//        // NW
//        assert_eq!(
//            SpiralArray::get_quardrant(Position { x: 0, y: -1 }),
//            Quadrant::NW
//        );
//        assert_eq!(
//            SpiralArray::get_quardrant(Position { x: -1, y: -1 }),
//            Quadrant::NW
//        );
//        assert_eq!(
//            SpiralArray::get_quardrant(Position { x: -1, y: -2 }),
//            Quadrant::NW
//        );
//        // W
//        assert_eq!(
//            SpiralArray::get_quardrant(Position { x: -1, y: 0 }),
//            Quadrant::W
//        );
//        assert_eq!(
//            SpiralArray::get_quardrant(Position { x: -2, y: 1 }),
//            Quadrant::W
//        );
//        assert_eq!(
//            SpiralArray::get_quardrant(Position { x: -3, y: 1 }),
//            Quadrant::W
//        );
//        // SW
//        assert_eq!(
//            SpiralArray::get_quardrant(Position { x: -1, y: 1 }),
//            Quadrant::SW
//        );
//        assert_eq!(
//            SpiralArray::get_quardrant(Position { x: -1, y: 2 }),
//            Quadrant::SW
//        );
//        assert_eq!(
//            SpiralArray::get_quardrant(Position { x: -2, y: 3 }),
//            Quadrant::SW
//        );
//        // SE
//        assert_eq!(
//            SpiralArray::get_quardrant(Position { x: 0, y: 1 }),
//            Quadrant::SE
//        );
//        assert_eq!(
//            SpiralArray::get_quardrant(Position { x: 1, y: 1 }),
//            Quadrant::SE
//        );
//        assert_eq!(
//            SpiralArray::get_quardrant(Position { x: 1, y: 2 }),
//            Quadrant::SE
//        );
//    }
//
//    #[test]
//    fn test_get_ring() {
//        let position = Position { x: -1, y: 0 };
//        assert_eq!(
//            SpiralArray::get_ring(SpiralArray::get_quardrant(position), position),
//            1
//        );
//        let position = Position { x: -1, y: -2 };
//        assert_eq!(
//            SpiralArray::get_ring(SpiralArray::get_quardrant(position), position),
//            3
//        );
//        let position = Position { x: 1, y: 1 };
//        assert_eq!(
//            SpiralArray::get_ring(SpiralArray::get_quardrant(position), position),
//            2
//        );
//    }
//
//    #[test]
//    fn test_get_quadrant_position() {
//        assert_eq!(
//            SpiralArray::get_quadrant_position(1, Quadrant::E),
//            Position { x: 1, y: 0 }
//        );
//        assert_eq!(
//            SpiralArray::get_quadrant_position(3, Quadrant::W),
//            Position { x: -3, y: 0 }
//        );
//        assert_eq!(
//            SpiralArray::get_quadrant_position(2, Quadrant::NE),
//            Position { x: 2, y: -2 }
//        );
//        assert_eq!(
//            SpiralArray::get_quadrant_position(3, Quadrant::SW),
//            Position { x: -3, y: 3 }
//        );
//        assert_eq!(
//            SpiralArray::get_quadrant_position(1, Quadrant::NW),
//            Position { x: 0, y: -1 }
//        );
//        assert_eq!(
//            SpiralArray::get_quadrant_position(2, Quadrant::SE),
//            Position { x: 0, y: 2 }
//        );
//    }
//
//    #[test]
//    fn test_quadrant_start() {
//        assert_eq!(
//            SpiralArray::quadrant_start(Position { x: -1, y: 0 }),
//            (Position { x: -1, y: 0 }, 4)
//        );
//        assert_eq!(
//            SpiralArray::quadrant_start(Position { x: -2, y: 2 }),
//            (Position { x: -2, y: 2 }, 15)
//        );
//        assert_eq!(
//            SpiralArray::quadrant_start(Position { x: -3, y: 2 }),
//            (Position { x: -3, y: 0 }, 28)
//        );
//        assert_eq!(
//            SpiralArray::quadrant_start(Position { x: 4, y: 0 }),
//            (Position { x: 4, y: 0 }, 37)
//        );
//    }
//
//    #[test]
//    fn test_spiral_array_position() {
//        // Center
//        assert_eq!(SpiralArray::position(Position { x: 0, y: 0 }), 0);
//        // E
//        assert_eq!(SpiralArray::position(Position { x: 1, y: 0 }), 1);
//        assert_eq!(SpiralArray::position(Position { x: 2, y: 0 }), 7);
//        assert_eq!(SpiralArray::position(Position { x: 3, y: -1 }), 20);
//        // NE
//        assert_eq!(SpiralArray::position(Position { x: 1, y: -1 }), 2);
//        assert_eq!(SpiralArray::position(Position { x: 1, y: -2 }), 10);
//        assert_eq!(SpiralArray::position(Position { x: 2, y: -3 }), 23);
//        // NW
//        assert_eq!(SpiralArray::position(Position { x: 0, y: -1 }), 3);
//        assert_eq!(SpiralArray::position(Position { x: -1, y: -1 }), 12);
//        assert_eq!(SpiralArray::position(Position { x: -2, y: -1 }), 27);
//        // W
//        assert_eq!(SpiralArray::position(Position { x: -1, y: 0 }), 4);
//        assert_eq!(SpiralArray::position(Position { x: -2, y: 1 }), 14);
//        assert_eq!(SpiralArray::position(Position { x: -3, y: 1 }), 29);
//        // SW
//        assert_eq!(SpiralArray::position(Position { x: -1, y: 1 }), 5);
//        assert_eq!(SpiralArray::position(Position { x: -1, y: 2 }), 16);
//        assert_eq!(SpiralArray::position(Position { x: -2, y: 3 }), 32);
//        // SE
//        assert_eq!(SpiralArray::position(Position { x: 0, y: 1 }), 6);
//        assert_eq!(SpiralArray::position(Position { x: 0, y: 2 }), 17);
//        assert_eq!(SpiralArray::position(Position { x: 2, y: 1 }), 36);
//    }
//}
