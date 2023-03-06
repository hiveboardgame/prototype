use crate::game_result::GameResult;

#[derive(thiserror::Error, Debug)]
pub enum GameError {
    #[error("Not a valid game move: {reason} turn: {turn}, piece: {piece}, current_position: {from}, target_position: {to}.")]
    InvalidMove {
        piece: String,
        from: String,
        to: String,
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
