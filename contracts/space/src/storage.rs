use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdError, StdResult};
use cw_storage_plus::Map;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DataEntry {
    pub key: String,
    pub owner: String,
    pub encrypted_data: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    StoreData {
        key: String,
        data: Vec<u8>,
        public_key: String,
    },
    DeleteData {
        key: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetData { key: String },
}

pub const DATA_STORE: Map<String, DataEntry> = Map::new("data_store");

pub fn encrypt_data(public_key: &str, data: &[u8]) -> StdResult<Vec<u8>> {
    // Placeholder encryption logic; replace with proper cryptographic library
    Ok(data.iter().map(|byte| byte ^ 0xAA).collect())
}

pub fn decrypt_data(private_key: &str, encrypted_data: &[u8]) -> StdResult<Vec<u8>> {
    // Placeholder decryption logic; replace with proper cryptographic library
    Ok(encrypted_data.iter().map(|byte| byte ^ 0xAA).collect())
}

pub fn store_data(
    deps: DepsMut,
    info: MessageInfo,
    key: String,
    data: Vec<u8>,
    public_key: String,
) -> StdResult<Response> {
    let encrypted_data = encrypt_data(&public_key, &data)?;
    let entry = DataEntry {
        key: key.clone(),
        owner: info.sender.to_string(),
        encrypted_data,
    };
    DATA_STORE.save(deps.storage, key, &entry)?;
    Ok(Response::new().add_attribute("action", "store_data"))
}

pub fn delete_data(
    deps: DepsMut,
    info: MessageInfo,
    key: String,
) -> StdResult<Response> {
    let entry = DATA_STORE.load(deps.storage, key)?;
    if entry.owner != info.sender.to_string() {
        return Err(StdError::unauthorized());
    }
    DATA_STORE.remove(deps.storage, &key);
    Ok(Response::new().add_attribute("action", "delete_data"))
}

pub fn query_data(
    deps: DepsMut,
    key: String,
    private_key: String,
) -> StdResult<Vec<u8>> {
    let entry = DATA_STORE.load(deps.storage, &key)?;
    let decrypted_data = decrypt_data(&private_key, &entry.encrypted_data)?;
    Ok(decrypted_data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_info};

    #[test]
    fn test_store_and_query_data() {
        let mut deps = mock_dependencies();
        let info = mock_info("creator", &[]);
        let key = "test_key".to_string();
        let data = b"test_data".to_vec();
        let public_key = "public_key".to_string();
        let private_key = "private_key".to_string();

        // Store data
        let response = store_data(deps.as_mut(), info.clone(), key.clone(), data.clone(), public_key.clone()).unwrap();
        assert_eq!(response.attributes[0].value, "store_data");

        // Query data
        let result = query_data(deps.as_mut(), key.clone(), private_key).unwrap();
        assert_eq!(result, data);
    }

    #[test]
    fn test_delete_data() {
        let mut deps = mock_dependencies();
        let info = mock_info("creator", &[]);
        let key = "test_key".to_string();
        let data = b"test_data".to_vec();
        let public_key = "public_key".to_string();

        // Store data
        store_data(deps.as_mut(), info.clone(), key.clone(), data.clone(), public_key.clone()).unwrap();

        // Delete data
        let response = delete_data(deps.as_mut(), info.clone(), key.clone()).unwrap();
        assert_eq!(response.attributes[0].value, "delete_data");
    }
}