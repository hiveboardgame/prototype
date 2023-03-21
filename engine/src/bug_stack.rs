use crate::piece::Piece;
use crate::color::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BugStack {
    pub pieces: [Piece; 7],
    pub size: u8,
}

impl BugStack {
    pub fn new() -> Self {
        Self {
            pieces: [Piece::new(); 7],
            size: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.size as usize
    }

    pub fn empty(&self) -> bool {
        self.size == 0
    }

    pub fn top_bug_color(&self) -> Option<Color> {
        if self.empty() {
            return None;
        }
        Some(self.pieces[self.size as usize].color())
    }

    pub fn push_piece(&mut self, piece: Piece) {
        if self.size == 7 {
            panic!("Trying to add an 8th bug to a BugStack")
        }
        self.pieces[self.size as usize] = piece;
        self.size += 1;
    }

    pub fn pop_piece(&mut self) -> Piece {
        if self.size == 0 {
            panic!("Trying to remove a bug from an empty BugStack")
        }
        self.size -= 1;
        let piece = self.pieces[self.size as usize];
        self.pieces[self.size as usize] = Piece::new();
        piece
    }

    pub fn top_piece(&self) -> Option<Piece> {
        if self.size == 0 {
            return None;
        }
        Some(self.pieces[(self.size - 1) as usize])
    }
}
