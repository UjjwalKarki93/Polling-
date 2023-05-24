use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Setup {
    pub owner: Addr,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Poll {
    pub question: String,
    pub yes: u64,
    pub no: u64,
}

pub const SETUP: Item<Setup> = Item::new("state");

//mapping question(string) -> poll
pub const POLL: Map<String,Poll> = Map::new("poll");
