use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Node {
    pub id: u64,
    pub data: String,
    pub links: Vec<u64>,
}

pub const NODES: Map<u64, Node> = Map::new("nodes");
pub const NEXT_NODE_ID: Item<u64> = Item::new("next_node_id");