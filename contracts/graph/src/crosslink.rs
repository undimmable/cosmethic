use cosmwasm_std::{entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Node, NODES, NEXT_NODE_ID};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    NEXT_NODE_ID.save(deps.storage, &0).map_err(|e| e.to_string())?;
    Ok(Response::new())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::CreateNode { data } => create_node(deps, info, data),
        ExecuteMsg::UpdateNode { node_id, data } => update_node(deps, info, node_id, data),
        ExecuteMsg::CreateLink { from_node_id, to_node_id } => create_link(deps, info, from_node_id, to_node_id),
    }
}

fn create_node(deps: DepsMut, _info: MessageInfo, data: String) -> StdResult<Response> {
    let node_id = NEXT_NODE_ID.load(deps.storage).map_err(|e| e.to_string())?;
    let new_node = Node {
        id: node_id,
        data,
        links: vec![],
    };
    NODES.save(deps.storage, node_id, &new_node).map_err(|e| e.to_string())?;
    NEXT_NODE_ID.save(deps.storage, &(node_id + 1)).map_err(|e| e.to_string())?;
    Ok(Response::new()
        .add_attribute("action", "create_node")
        .add_attribute("node_id", node_id.to_string()))
}

fn update_node(deps: DepsMut, _info: MessageInfo, node_id: u64, data: String) -> StdResult<Response> {
    let mut node = NODES.load(deps.storage, node_id).map_err(|e| e.to_string())?;
    node.data = data;
    NODES.save(deps.storage, node_id, &node).map_err(|e| e.to_string())?;
    Ok(Response::new()
        .add_attribute("action", "update_node")
        .add_attribute("node_id", node_id.to_string()))
}

fn create_link(deps: DepsMut, _info: MessageInfo, from_node_id: u64, to_node_id: u64) -> StdResult<Response> {
    let mut node = NODES.load(deps.storage, from_node_id).map_err(|e| e.to_string())?;
    node.links.push(to_node_id);
    NODES.save(deps.storage, from_node_id, &node).map_err(|e| e.to_string())?;
    Ok(Response::new().add_attribute("action", "create_link").add_attribute("from_node_id", from_node_id.to_string()).add_attribute("to_node_id", to_node_id.to_string()))
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetNode { node_id } => to_binary(&query_node(deps, node_id).map_err(|e| e.to_string())?),
    }
}

fn query_node(deps: Deps, node_id: u64) -> StdResult<Node> {
    let node = NODES.load(deps.storage, node_id).map_err(|e| e.to_string())?;
    Ok(node)
}