use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
use crate::msg::{GameMove, GameResult};


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub count: i32,
    pub owner: Addr,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Data{
    pub opponent: Addr,
    pub host_move: GameMove,
    pub opp_move:  GameMove,
    pub result: GameResult,
}


pub const STATE: Item<State> = Item::new("state");
pub const ENTRIES: Map<Addr, Data> = Map::new("entries");
