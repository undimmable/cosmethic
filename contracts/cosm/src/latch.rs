use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdError, StdResult};
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub roles: Map<String, String>, // Role assignments (address -> role)
    pub latch_actors: Map<String, String>, // Sync state for latch actors
}

// State storage
pub const LATCHES: Map<String, Latch> = Map::new("latches");
pub const CONFIG: Item<Config> = Item::new("config");

// Create a new latch
pub fn create_latch(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    id: String,
    threshold: u64,
    participants: Vec<String>,
) -> StdResult<Response> {
    let latch = Latch {
        id: id.clone(),
        threshold,
        votes: 0,
        participants: participants.clone(),
        is_open: false,
    };

    // Save latch
    LATCHES.save(deps.storage, &id, &latch)?;
    Ok(Response::new().add_attribute("method", "create_latch").add_attribute("latch_id", id))
}

// Cast a vote
pub fn vote(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    latch_id: String,
) -> StdResult<Response> {
    let mut latch = LATCHES.load(deps.storage, &latch_id)?;

    // Ensure the voter is a participant
    if !latch.participants.contains(&info.sender.to_string()) {
        return Err(StdError::generic_err("Unauthorized participant"));
    }

    // Increment votes
    latch.votes += 1;

    // Check if threshold is reached
    if latch.votes >= latch.threshold {
        latch.is_open = true;
    }

    // Save latch state
    LATCHES.save(deps.storage, &latch_id, &latch)?;

    Ok(Response::new()
        .add_attribute("method", "vote")
        .add_attribute("latch_id", latch_id)
        .add_attribute("is_open", latch.is_open.to_string()))
}

// Update latch threshold dynamically
pub fn update_threshold(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    latch_id: String,
    new_threshold: u64,
) -> StdResult<Response> {
    let mut latch = LATCHES.load(deps.storage, &latch_id)?;

    // Ensure sender has a role that allows threshold modification
    let config = CONFIG.load(deps.storage)?;
    let role = config.roles.get(&info.sender.to_string()).cloned();
    if role != Some("admin".to_string()) {
        return Err(StdError::generic_err("Unauthorized"));
    }

    // Update threshold
    latch.threshold = new_threshold;
    LATCHES.save(deps.storage, &latch_id, &latch)?;

    Ok(Response::new()
        .add_attribute("method", "update_threshold")
        .add_attribute("latch_id", latch_id)
        .add_attribute("new_threshold", new_threshold.to_string()))
}

// Synchronize latch state across actors
pub fn sync_latch(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    latch_id: String,
    actor_state: String,
) -> StdResult<Response> {
    let mut config = CONFIG.load(deps.storage)?;

    // Update latch actor state
    config.latch_actors.insert(latch_id.clone(), actor_state);
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("method", "sync_latch")
        .add_attribute("latch_id", latch_id)
        .add_attribute("actor_state", actor_state))
}

// Initialize the contract with roles and actors
pub fn init_contract(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    admin: String,
) -> StdResult<Response> {
    let config = Config {
        roles: Map::new("roles"),
        latch_actors: Map::new("latch_actors"),
    };
    CONFIG.save(deps.storage, &config)?;

    // Assign admin role
    CONFIG.update(deps.storage, |mut config| -> StdResult<_> {
        config.roles.insert(admin.clone(), "admin".to_string());
        Ok(config)
    })?;

    Ok(Response::new().add_attribute("method", "init_contract").add_attribute("admin", admin))
}
