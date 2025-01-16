use std::fmt::Binary;
use crate::msg::{ExecuteMsg, FilesResponse, InstantiateMsg, QueryMsg};
use crate::state::{load_config, load_files, save_config, save_file, State};
use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let state = State { admin: deps.api.addr_validate(&msg.admin)? };
    save_config(deps.storage, &state)?;
    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    let state = load_config(deps.storage)?;
    if info.sender != state.admin {
        return Err(cosmwasm_std::StdError::generic_err("Unauthorized"));
    }

    match msg {
        ExecuteMsg::AddFile { hash } => {
            save_file(deps.storage, &hash)?;
            Ok(Response::default())
        }
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetFiles {} => {
            let files = load_files(deps.storage)?;
            to_binary(&FilesResponse { hashes: files })
        }
    }
}