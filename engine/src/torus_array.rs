use crate::{position::Position, board::BOARD_SIZE};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq)]
pub struct TorusArray<T> {
    data: Vec<T>,
    default: T,
}

impl<T> TorusArray<T>
where
    T: Clone,
{
    pub fn new(default: T) -> Self {
        Self {
            data: vec![default.clone(); (BOARD_SIZE * BOARD_SIZE) as usize],
            default,
        }
    }

    pub fn get(&self, position: Position) -> &T {
        self.data.get((position.y * BOARD_SIZE + position.x) as usize).expect(
            "TorusArray found an empty position, this should not happen because it's initialized",
        )
    }

    pub fn get_mut(&mut self, position: Position) -> &mut T {
        self.data.get_mut((position.y * BOARD_SIZE + position.x) as usize).expect(
            "TorusArray found an empty position, this should not happen because it's initialized",
        )
    }

    // TODO get rid of this
    pub fn remove(&mut self, position: Position) {
        self.set(position, self.default.clone());
    }

    pub fn set(&mut self, position: Position, element: T) {
        self.data[(position.y * BOARD_SIZE + position.x) as usize] = element;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new_insert_get() {
        let mut arr = TorusArray::new(0_i32);
        let position = Position { x: 0, y: 1 };
        arr.set(position, 1);
        assert_eq!(*arr.get(position), 1);
    }
}
