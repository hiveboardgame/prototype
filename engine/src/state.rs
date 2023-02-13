use crate::bug::Bug;
use crate::color::Color;
use crate::game_result::GameResult;
use crate::hasher::Hasher;
use crate::history::History;
use crate::piece::Piece;
use crate::player::Player;
use crate::position::Position;
use crate::{board::Board, game_type::GameType};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum LastTurn {
    Pass,
    Shutout,
    Move(Position, Position),
    None,
}

impl Default for LastTurn {
    fn default() -> Self {
        LastTurn::None
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct State {
    pub board: Board,
    pub history: History,
    pub hasher: Hasher,
    pub last_turn: LastTurn,
    pub turn: usize,
    pub turn_color: Color,
    pub players: (Player, Player),
    pub game_result: GameResult,
    pub game_type: GameType,
}

impl State {
    pub fn new(game_type: GameType) -> State {
        State {
            board: Board::new(),
            history: History::new(),
            hasher: Hasher::new(),
            last_turn: LastTurn::None,
            turn: 0,
            turn_color: Color::White,
            players: (Player::new(Color::White), Player::new(Color::Black)),
            game_result: GameResult::Unknown,
            game_type,
        }
    }

    pub fn new_from_history(history: &History) -> Self {
        let mut state = State::new(history.game_type);
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
        match piece {
            "pass" => {
                if self.last_turn == LastTurn::Shutout {
                    self.last_turn = LastTurn::Pass;
                    // we handled this in shutout already
                    // Don't do anything
                } else {
                    panic!(
                        "\nProcessing turn: #{}\nThis is not a valid pass!",
                        self.turn
                    );
                }
            }
            _ => {
                let piece = Piece::from_string(piece);
                let target_position = Position::from_string(position, &self.board);
                self.play_turn(piece, target_position);
            }
        }
    }

    fn update_history(&mut self, piece: Piece, target_position: Position) {
        // if there's no piece on the board yet use "."
        let mut pos = ".".to_string();
        if self
            .board
            .board
            .get(&target_position)
            .unwrap_or(&vec![])
            .len()
            > 1
        {
            let pieces = self.board.board.get(&target_position).unwrap();
            let len = pieces.len();
            let second_to_last = pieces[len - 2];
            pos = second_to_last.to_string();
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
        let no_spawns = !self.board.spawns_left(&self.turn_color, self.game_type);
        let no_moves = self
            .board
            .moves(&self.turn_color)
            .values()
            .flatten()
            .collect::<Vec<&Position>>()
            .is_empty();
        if no_moves && no_spawns {
            self.pass();
            self.last_turn = LastTurn::Shutout;
        }
    }

    fn pass(&mut self) {
        self.history
            .record_move(self.turn_color.to_string(), "pass".to_string());
        self.turn_color = self.turn_color.opposite();
        self.turn += 1;
        self.board.last_moved = None;
        self.update_hasher();
    }

    fn next_turn(&mut self) {
        self.game_result = self.board.game_result();
        match self.game_result {
            GameResult::Winner(color) => {
                self.history
                    .record_move(color.to_string(), "won".to_string());
                return;
            }
            GameResult::Draw => {
                self.history
                    .record_move("It's a draw".to_string(), "".to_string());
                return;
            }
            _ => {}
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
                println!("Board state is: \n{}", self.board);
                panic!(
                    "Not a legal move! \n Turn is {:?} color is {:?}\n Trying to move {piece} from {current_position} to
                       {target_position} \n But valid target positions are only: {:?}\n All valid moves are: {:?}",
                       self.turn, self.turn_color, moves.get(&(piece, current_position)).unwrap_or(&Vec::new()), moves
                );
            }
            self.last_turn = LastTurn::Move(current_position, target_position);
            self.board
                .move_piece(&piece, &current_position, &target_position);
        } else {
            // let's spawn the piece
            if piece.bug != Bug::Queen && self.board.queen_required(self.turn, &piece.color) {
                panic!("Queen needs to be spawned");
            }
            if self.board.spawnable(&piece.color, &target_position) {
                self.board.insert(&target_position, piece);
                self.last_turn = LastTurn::Move(target_position, target_position);
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
