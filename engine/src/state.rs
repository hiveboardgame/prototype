use crate::bug::Bug;
use crate::color::Color;
use crate::game_error::GameError;
use crate::game_result::GameResult;
use crate::hasher::Hasher;
use crate::history::History;
use crate::piece::Piece;
use crate::player::Player;
use crate::position::Position;
use crate::{board::Board, game_type::GameType};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum LastTurn {
    Pass,
    Shutout,
    Move(Position, Position),
    #[default]
    None,
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
    pub tournament: bool,
}

impl State {
    pub fn new(game_type: GameType, tournament: bool) -> State {
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
            tournament,
        }
    }

    pub fn new_from_history(history: &History) -> Result<Self, GameError> {
        let mut tournament = true;
        // Did white open with a Queen?
        if let Some((piece_str, _)) = history.moves.get(0) {
            let piece: Piece = piece_str.parse()?;
            if piece.bug == Bug::Queen {
                tournament = false;
            }
        }
        // Did black open with a Queen?
        if let Some((piece_str, _)) = history.moves.get(1) {
            let piece: Piece = piece_str.parse()?;
            if piece.bug == Bug::Queen {
                tournament = false;
            }
        }
        let mut state = State::new(history.game_type, tournament);
        state.history = history.clone();
        for (piece, pos) in history.moves.iter() {
            state.play_turn_from_notation(piece, pos)?;
        }
        Ok(state)
    }

    pub fn queen_allowed(&self) -> bool {
        self.turn > 1 || !self.tournament
    }

    pub fn play_turn_from_notation(
        &mut self,
        piece: &str,
        position: &str,
    ) -> Result<(), GameError> {
        match piece {
            "pass" => {
                if self.last_turn == LastTurn::Shutout {
                    self.last_turn = LastTurn::Pass;
                    // we handled this in shutout already
                    // Don't do anything
                } else {
                    return Err(GameError::InvalidMove {
                        piece: "NA".to_string(),
                        from: "NA".to_string(),
                        to: "NA".to_string(),
                        turn: self.turn,
                        reason: "Trying to pass when there are available moves".to_string(),
                    });
                }
            }
            _ => {
                let piece = piece.parse()?;
                let target_position = Position::from_string(position, &self.board)?;
                self.play_turn(piece, target_position)?;
            }
        }
        Ok(())
    }

    fn update_history(&mut self, piece: &Piece, target_position: &Position) {
        // if there's no piece on the board yet use "."
        let mut pos = ".".to_string();
        if let Some(top_piece) = self.board.top_piece(target_position) {
            pos = top_piece.to_string();
        } else {
            // no piece at the current position, so it's a spawn or a move
            if let Some((neighbor_piece, neighbor_pos)) = self.board.get_neighbor(target_position) {
                let dir = neighbor_pos.direction(target_position);
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

    pub fn play_turn(&mut self, piece: Piece, target_position: Position) -> Result<(), GameError> {
        // If the piece is already in play, it's a move
        if self.board.piece_already_played(&piece) {
            let current_position = self.board.position(&piece).ok_or(GameError::InvalidMove {
                piece: piece.to_string(),
                from: "NA".to_string(),
                to: target_position.to_string(),
                turn: self.turn,
                reason: "This piece is not on the board".to_string(),
            })?;
            if self.board.pinned(&current_position) {
                return Err(GameError::InvalidMove {
                    piece: piece.to_string(),
                    from: current_position.to_string(),
                    to: target_position.to_string(),
                    turn: self.turn,
                    reason: "Piece is pinned".to_string(),
                });
            }
            // remove the piece from its current location
            if !self.board.is_valid_move(
                &self.turn_color,
                &piece,
                &current_position,
                &target_position,
            ) {
                return Err(GameError::InvalidMove {
                    piece: piece.to_string(),
                    from: current_position.to_string(),
                    to: target_position.to_string(),
                    turn: self.turn,
                    reason: "This move isn't valid.".to_string(),
                });
            }
            self.last_turn = LastTurn::Move(current_position, target_position);
            self.board
                .move_piece(&piece, &current_position, &target_position, self.turn)?;
        } else {
            // Handle spawns
            if !piece.is_color(&self.turn_color) {
                return Err(GameError::InvalidMove {
                    piece: piece.to_string(),
                    from: "NA".to_string(),
                    to: target_position.to_string(),
                    turn: self.turn,
                    reason: format!(
                        "It is {}'s turn, but {} tried to spawn a piece",
                        self.turn_color, piece.color
                    ),
                });
            }
            if piece.bug != Bug::Queen && self.board.queen_required(self.turn, &piece.color) {
                return Err(GameError::InvalidMove {
                    piece: piece.to_string(),
                    from: "Reserve".to_string(),
                    to: target_position.to_string(),
                    turn: self.turn,
                    reason: "Can't spawn another piece. Queen is required".to_string(),
                });
            }
            if self.board.spawnable(&piece.color, &target_position) {
                self.board.insert(&target_position, piece);
                self.last_turn = LastTurn::Move(target_position, target_position);
            } else {
                return Err(GameError::InvalidMove {
                    piece: piece.to_string(),
                    from: "Reserve".to_string(),
                    to: target_position.to_string(),
                    turn: self.turn,
                    reason: format!("{} is not allowed to spawn here", self.turn_color),
                });
            }
        }
        self.update_history(&piece, &target_position);
        self.update_hasher();
        self.next_turn();
        self.shutout();
        Ok(())
    }
}
