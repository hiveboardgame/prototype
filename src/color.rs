use std::fmt;

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
pub enum Color {
    Black,
    White,
}

impl Color {
    pub fn opposite(&self) -> Color {
        match self {
            Self::Black => Self::White,
            Self::White => Self::Black,
        }
    }

    pub fn from_str(s: &str) -> Color {
        match s {
            "w" => Color::White,
            "b" => Color::Black, 
            _ => panic!("That's not a color!"),
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let color = match self {
            Color::White => "w",
            Color::Black => "b",
        };
        write!(f, "{}", color)
    }
}
