use crate::position::Position;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq)]
pub struct TorusArray<T> {
    data: Vec<T>,
    width: i32,
    height: i32,
    default: T,
}

impl<T> TorusArray<T>
where
    T: Clone,
{
    pub fn new(width: usize, height: usize, default: T) -> Self {
        Self {
            data: vec![default.clone(); width * height],
            width: width.try_into().expect("too big"),
            height: height.try_into().expect("too big"),
            default,
        }
    }

    pub fn get(&self, position: Position) -> &T {
        // TODO Move modulo this into position
        // assert!(position.x < self.width);
        // assert!(position.y < self.height);
        let x = position.x.rem_euclid(self.width) as usize;
        let y = position.y.rem_euclid(self.height) as usize;
        self.data.get(y * (self.width as usize) + x).expect(
            "TorusArray found an empty position, this should not happen because it's initialized",
        )
    }

    pub fn get_mut(&mut self, position: Position) -> &mut T {
        let x = position.x.rem_euclid(self.width) as usize;
        let y = position.y.rem_euclid(self.height) as usize;
        self.data.get_mut(y * (self.width as usize) + x).expect(
            "TorusArray found an empty position, this should not happen because it's initialized",
        )
    }

    // TODO get rid of this
    pub fn remove(&mut self, position: Position) {
        self.set(position, self.default.clone());
    }

    pub fn set(&mut self, position: Position, element: T) {
        let x = position.x.rem_euclid(self.width) as usize;
        let y = position.y.rem_euclid(self.height) as usize;
        self.data[y * (self.width as usize) + x] = element;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new_insert_get() {
        let mut arr = TorusArray::new(32, 32, 0_i32);
        let position = Position { x: 0, y: 1 };
        arr.set(position, 1);
        assert_eq!(*arr.get(position), 1);
    }
}
