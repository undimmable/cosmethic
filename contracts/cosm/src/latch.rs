use crate::error::ContractError;
use crate::state::{Config, Latch, CONFIG, LATCHES};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdError, StdResult};
use cw_storage_plus::Map;

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
    Ok(Response::new()
        .add_attribute("method", "create_latch")
        .add_attribute("latch_id", id))
}

// Cast a vote
pub fn vote(deps: DepsMut, _env: Env, info: MessageInfo, latch_id: String) -> StdResult<Response> {
    let mut latch = LATCHES.load(deps.storage, latch_id.clone())?;

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
    LATCHES.save(deps.storage, latch_id.clone(), &latch)?;

    Ok(Response::new()
        .add_attribute("method", "vote")
        .add_attribute("latch_id", latch_id.clone())
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
    let mut latch = LATCHES.load(deps.storage, latch_id.clone())?;

    // Ensure sender has a role that allows threshold modification
    let config = CONFIG.load(deps.storage)?;
    let role = config.roles.get(&info.sender.to_string()).cloned();
    if role != Some("admin".to_string()) {
        return Err(StdError::generic_err("Unauthorized"));
    }

    // Update threshold
    latch.threshold = new_threshold;
    LATCHES.save(deps.storage, latch_id.clone(), &latch)?;

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
    let result = config
        .latch_actors
        .save(deps.storage, latch_id.clone(), &actor_state)?;

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("method", "sync_latch")
        .add_attribute("latch_id", latch_id.clone())
        .add_attribute("actor_state", actor_state.clone()))
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
        //I don't undestand what you meant here, Freyja. Did you want to store contract owner as a role? Then we need to separate Roles from Species
        config.roles.save(deps.storage, admin.clone());
        Ok(config)
    })?;

    Ok(Response::new()
        .add_attribute("method", "init_contract")
        .add_attribute("admin", admin.clone()))
}
