use crate::board::Board;
use crate::color::Color;
use crate::hasher::Hasher;
use crate::history::History;
use crate::piece::Piece;
use crate::player::Player;
use crate::position::Position;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct State {
    pub board: Board,
    pub history: History,
    pub hasher: Hasher,
    pub turn: i32,
    pub turn_color: Color,
    pub players: (Player, Player),
}

impl State {
    pub fn new() -> State {
        State {
            board: Board::new(),
            history: History::new(),
            hasher: Hasher::new(),
            turn: 0,
            turn_color: Color::White,
            players: (Player::new(Color::White), Player::new(Color::Black)),
        }
    }

    pub fn new_from_history(history: &History) -> Self {
        let mut state = State::new();
        state.history = history.clone();
        for (piece, pos) in history.moves.iter() {
            state.play_turn(piece, pos);
            let mut h = History::new();
            h.moves = history.moves[0..=((state.turn - 1) as usize)].to_vec();
            state.hasher.record_move(&h);
            state.hasher.record_board_state(&state.board);
        }
        state
    }

    pub fn play_turn(&mut self, piece: &str, position: &str) {
        let piece = Piece::from_string(piece);
        let target_position = Position::from_string(position, &self.board);
        let moves = self.board.moves(&self.turn_color);

        // If the piece is already in play, it's a move
        if self.board.piece_already_played(&piece) {
            let current_position = self.board.position(&piece);
            if self.board.pinned(&current_position) {
                panic!(
                    "Invalid history: Piece {} at pos {} is pinned!",
                    piece, current_position
                );
            }
            // remove the piece from its current location
            if !self.board.is_valid_move(&self.turn_color, &piece, &current_position, &target_position) {
                println!("Trying to move {piece} from {current_position} to {target_position}");
                println!(
                    "But valid target positions are only: {:?}",
                        moves
                        .get(&(piece, current_position))
                        .unwrap_or(&Vec::new())
                );
                panic!("Not a legal move!");
            }
            self.board
                .move_piece(&piece, &current_position, &target_position);
        } else {
            // let's spawn the piece
            if self.board.spawnable(&piece.color, &target_position) {
                self.board.insert(&target_position, piece);
            } else {
                panic!("Can't spawn here!");
            }
        }
        self.turn += 1;
        self.turn_color = self.turn_color.opposite();
        // valid?
        // write history
        // update hasher
        // check for win
    }
}
