
pub fn store_path_embedding(
    deps: DepsMut,
    path: PathEmbedding,
) -> Result<Response, ContractError> {
    PATH_STORAGE.save(deps.storage, path.session_id.clone(), &path)?;
    Ok(Response::new().add_attribute("stored_path", path.session_id))
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        from_binary,
        testing::{mock_dependencies, mock_env, mock_info},
    };
}
