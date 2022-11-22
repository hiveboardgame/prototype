use crate::board::Board;
use crate::bug::Bug;
use crate::color::Color;
use crate::hasher::Hasher;
use crate::history::History;
use crate::piece::Piece;
use crate::player::Player;
use crate::position::Position;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct State {
    pub board: Board,
    pub history: History,
    pub hasher: Hasher,
    pub last_turn: Option<(Position, Position)>,
    pub turn: usize,
    pub turn_color: Color,
    pub players: (Player, Player),
    pub winner: Option<Color>,
}

impl State {
    pub fn new() -> State {
        State {
            board: Board::new(),
            history: History::new(),
            hasher: Hasher::new(),
            last_turn: None,
            turn: 0,
            turn_color: Color::White,
            players: (Player::new(Color::White), Player::new(Color::Black)),
            winner: None,
        }
    }

    pub fn new_from_history(history: &History) -> Self {
        let mut state = State::new();
        state.history = history.clone();
        for (piece, pos) in history.moves.iter() {
            state.play_turn_from_notation(piece, pos);
        }
        state
    }

    pub fn queen_allowed(&self) -> bool {
        self.turn > 1
    }

    pub fn play_turn_from_notation(&mut self, piece: &str, position: &str) {
        let piece = Piece::from_string(piece);
        let target_position = Position::from_string(position, &self.board);
        self.play_turn(piece, target_position);
    }

    fn update_history(&mut self, piece: Piece, target_position: Position) {
        // if there's no piece on the board yet use "."
        let mut pos = ".".to_string();
        if let Some(pieces) = self.board.board.get(&target_position) {
            // there's a piece already at the position, so it must be a climb
            pos = pieces.last().unwrap().to_string();
        } else {
            // no piece at the current position, so it's a spawn or a move
            if let Some(neighbor_pos) = self.board.positions_taken_around(&target_position).get(0) {
                let neighbor_piece = self.board.top_piece(neighbor_pos);
                //let dir = neighbor_pos.direction(&target_position);
                let dir = neighbor_pos.direction(&target_position);
                pos = dir.to_history_string(neighbor_piece.to_string());
            }
        }
        self.history.record_move(piece.to_string(), pos);
    }

    fn shutout(&mut self) {
        let spawns = self.board.spawnable_positions(&self.turn_color).is_empty();
        let moves = self.board.moves(&self.turn_color).values().flatten().collect::<Vec<&Position>>().is_empty();
        if moves && spawns {
            self.history.record_move(self.turn_color.to_string(), "pass".to_string());
            self.turn_color = self.turn_color.opposite();
            self.turn += 1;
        }
        self.update_hasher();
    }

    fn next_turn(&mut self) {
        self.winner = self.board.winner();
        if let Some(winner) = self.winner {
            self.history.record_move(winner.to_string(), "won".to_string());
            return;
        }
        self.turn_color = self.turn_color.opposite();
        self.turn += 1;
    }

    fn update_hasher(&mut self) {
        let mut h = History::new();
        let mut turn = 0;
        if self.turn > 0 {
            turn = self.turn - 1;
        }
        h.moves = self.history.moves[0..=turn].to_vec();
        self.hasher.record_move(&h);
        self.hasher.record_board_state(&self.board);
    }

    pub fn play_turn(&mut self, piece: Piece, target_position: Position) {
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
            if !self.board.is_valid_move(
                &self.turn_color,
                &piece,
                &current_position,
                &target_position,
            ) {
                panic!(
                    "Not a legal move! \n Turn is {:?} color is {:?}\n Trying to move {piece} from {current_position} to
                       {target_position} \n But valid target positions are only: {:?}\n All valid moves are: {:?}",
                       self.turn, self.turn_color, moves.get(&(piece, current_position)).unwrap_or(&Vec::new()), moves
                );
            }
            self.last_turn = Some((current_position, target_position));
            self.board
                .move_piece(&piece, &current_position, &target_position);
        } else {
            // let's spawn the piece
            if piece.bug != Bug::Queen && self.board.queen_required(self.turn, &piece.color) {
                panic!("Queen needs to be spawned");
            }
            if self.board.spawnable(&piece.color, &target_position) {
                self.board.insert(&target_position, piece);
                self.last_turn = Some((target_position, target_position));
            } else {
                panic!("Can't spawn here!");
            }
        }
        self.update_history(piece, target_position);
        self.update_hasher();
        self.next_turn();
        self.shutout();
    }
}
