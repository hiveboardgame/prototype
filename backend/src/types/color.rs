use hive_lib::game_error::GameError;
use hive_lib::color::Color as OColor;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use std::convert::From;

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Clone, Copy, Debug, Default)]
#[repr(u8)]
pub enum Color {
    #[default]
    White = 0,
    Black = 1,
}

impl From<OColor> for Color {
    fn from(color: OColor) -> Self {
        match color {
            OColor::White => Color::White,
            OColor::Black => Color::Black,
        }
    }
}
