use hive_lib::state::State;
use yewdux::prelude::*;
use serde::{Serialize, Deserialize};


#[derive(Clone, Default, Serialize, Deserialize, Store, PartialEq)]
pub struct GameStateStore {
    pub state: State,
}

