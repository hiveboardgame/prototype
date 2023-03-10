use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum GameControl {
    AbortAccept,
    AbortOffer,
    AbortReject,
    DrawAccept,
    DrawOffer,
    DrawReject,
    Resign,
    TakebackAccept,
    TakebackOffer,
    TakebackReject,
}

impl fmt::Display for GameControl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let game_control = match self {
            GameControl::AbortAccept => "AbortAccept",
            GameControl::AbortOffer => "AbortOffer",
            GameControl::AbortReject => "AbortReject",
            GameControl::DrawAccept => "DrawAccept",
            GameControl::DrawOffer => "DrawOffer",
            GameControl::DrawReject => "DrawReject",
            GameControl::Resign => "Resign",
            GameControl::TakebackAccept => "TakebackAccept",
            GameControl::TakebackOffer => "TakebackOffer",
            GameControl::TakebackReject => "TakebackReject",
        };
        write!(f, "{}", game_control)
    }
}
