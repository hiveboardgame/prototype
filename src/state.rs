use crate::board::Board;
use crate::color::Color;
use crate::hasher::Hasher;
use crate::history::History;
use crate::moves::Moves;
use crate::piece::Piece;
use crate::player::Player;
use crate::position::Position;

pub struct State {
    pub board: Board,
    pub history: History,
    pub hasher: Hasher,
    pub turn: i32,
    pub players: (Player, Player),
}

impl State {
    pub fn new() -> State {
        State {
            board: Board::new(),
            history: History::new(),
            hasher: Hasher::new(),
            turn: 0,
            players: (Player::new(Color::White), Player::new(Color::Black)),
        }
    }

    pub fn new_from_history(history: &History) -> Self {
        let mut state = State::new();
        state.history = history.clone();
        for (i, (piece, pos)) in history.moves.iter().enumerate() {
            let piece = Piece::from_string(piece);
            let target_position = Position::from_string(pos, &state.board);
            let moves = Moves::new(i as i32, &state.board);

            // If the piece is already in play, it's a move
            if state.board.piece_already_played(&piece) {
                let current_position = state.board.position(&piece);
                if state.board.pinned(&current_position) {
                    panic!(
                        "Invalid history: Piece {} at pos {} is pinned!",
                        piece, current_position
                    );
                }
                // remove the piece from its current location
                if !moves.valid(&piece, &current_position, &target_position) {
                    println!("Trying to move {piece} from {current_position} to {target_position}");
                    println!(
                        "But valid target positions are only: {:?}",
                        moves
                            .moves
                            .get(&(piece, current_position))
                            .unwrap_or(&Vec::new())
                    );
                    panic!("Not a legal move!");
                }
                state
                    .board
                    .move_piece(&piece, &current_position, &target_position);
            } else {
                // let's spawn the piece
                if state.board.spawnable(&piece.color, &target_position) {
                    state.board.insert(&target_position, piece);
                } else {
                    panic!("Can't spawn here!");
                }
            }
            let mut h = History::new();
            h.moves = history.moves[0..=i].to_vec();
            state.hasher.record_move(&h);
            state.hasher.record_board_state(&state.board);
        }
        state
    }

    pub fn play_turn(&mut self) {
        // valid?
        // write history
        // update hasher
        // check for win
    }
}
