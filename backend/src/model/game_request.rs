use hive_lib::game_control::GameControl;
use serde::Deserialize;
use serde::Serialize;
use serde_with::serde_as;

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum GameRequest {
    Turn((String, String)),
    GameControl(GameControl),
}
