use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Latch {
    pub id: String,
    pub threshold: u64, // Required votes
    pub votes: u64,     // Current votes
    pub participants: Vec<String>, // Addresses of participants
    pub is_open: bool,  // Whether latch is open
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct Config {
    pub roles: Map<String, String>, // Role assignments (address -> role)
    pub latch_actors: Map<String, String>, // Sync state for latch actors
}

pub const LATCHES: Map<String, Latch> = Map::new("latches");
pub const CONFIG: Item<Config> = Item::new("config");
