use crate::Bug;
use crate::Color;
use std::collections::HashMap;

pub struct Player {
    color: Color,
    bugs: HashMap<Bug, i8>,
}

impl Player {
    pub fn new(color: Color) -> Player {
        Player {
            color,
            bugs: Bug::bugs_count(),
        }
    }

    fn use_bug(&mut self, bug: &Bug) {
        if let Some(num) = self.bugs.get_mut(bug) {
            *num -= 1;
        }
    }
}
