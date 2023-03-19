use crate::position::Position;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq)]
pub struct TorusArray<T> {
    data: Vec<T>,
    width: i32,
    height: i32,
}

impl<T> TorusArray<T>
where
    T: Clone,
{
    pub fn new(width: usize, height: usize, default: T) -> Self {
        Self {
            data: vec![default; width * height],
            width: width.try_into().expect("too big"),
            height: height.try_into().expect("too big"),
        }
    }

    pub fn get(&self, position: Position) -> &T {
        let x = position.x.rem_euclid(self.width) as usize;
        let y = position.y.rem_euclid(self.height) as usize;
        self.data.get(x * y + y).expect(
            "TorusArray found an empty position, this should not happen because it's initialized",
        )
    }

    pub fn get_mut(&mut self, position: Position) -> &mut T {
        let x = position.x.rem_euclid(self.width) as usize;
        let y = position.y.rem_euclid(self.height) as usize;
        self.data.get_mut(x * y + y).expect(
            "TorusArray found an empty position, this should not happen because it's initialized",
        )
    }

    pub fn set(&mut self, position: Position, element: T) {
        let x = position.x.rem_euclid(self.width) as usize;
        let y = position.y.rem_euclid(self.height) as usize;
        self.data.insert(x * y + y, element)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new_insert_get() {
        let mut arr = TorusArray::new(29, 29, 0_i32);
        let position = Position { x: 0, y: 1 };
        arr.set(position, 1);
        assert_eq!(*arr.get(position), 1);
    }
}
