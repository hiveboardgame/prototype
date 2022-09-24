use crate::board::Board;
use crate::player::Player;
use crate::color::Color;

pub struct State {
    pub board: Board,
    pub player: (Player, Player),
    pub history: Vec<String>,
}

impl State {
    pub fn new() -> State {
        State {
            board: Board::new(),
            player: (Player::new(Color::Black), Player::new(Color::White)),
            history: Vec::new(),
        }
    }
}
