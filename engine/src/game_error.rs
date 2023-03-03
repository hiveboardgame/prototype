use crate::game_result::GameResult;

#[derive(thiserror::Error, Debug)]
pub enum GameError {
    #[error("Invalid or illegal move on turn {turn}, moving piece {piece} from {from} to {to}")]
    InvalidMove {
        piece: String,
        from: String,
        to: String,
        turn: usize,
        reason: String,
    },
    #[error("Invalid spawn of piece {piece} at position {position} on turn {turn}")]
    InvalidSpawn {
        piece: String,
        position: String,
        turn: usize,
        reason: String,
    },
    #[error("Found {found:?} which is not a valid {typ}")]
    ParsingError { found: String, typ: String },
    #[error("Result {reported_result:?} doesn't match board endstate {actual_result:?}")]
    ResultMismatch {
        reported_result: GameResult,
        actual_result: GameResult,
    },
    #[error("No .pgn file supplied")]
    NoPgnFile,
    #[error("Invalid direction {direction:?}")]
    InvalidDirection { direction: String },
}
