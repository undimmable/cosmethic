use cosmwasm_std::{Addr, StdResult, Storage};
use cw_storage_plus::{singleton, singleton_read, Singleton};
use oasis_std::abi::{Deserialize, Serialize};
use schemars::JsonSchema;
use serde::ser::{Deserialize, Serialize};

static CONFIG_KEY: &[u8] = b"config";
static FILES_KEY: &[u8] = b"files";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub admin: Addr,
}

pub fn save_config(storage: &mut dyn Storage, state: &State) -> StdResult<()> {
    singleton(storage, CONFIG_KEY).save(state)
}

pub fn load_config(storage: &dyn Storage) -> StdResult<State> {
    singleton_read(storage, CONFIG_KEY).load()
}

pub fn save_file(storage: &mut dyn Storage, hash: &str) -> StdResult<()> {
    let mut files: Vec<String> = singleton_read(storage, FILES_KEY).may_load()?.unwrap_or_default();
    files.push(hash.to_string());
    singleton(storage, FILES_KEY).save(&files)
}

pub fn load_files(storage: &dyn Storage) -> StdResult<Vec<String>> {
    singleton_read(storage, FILES_KEY).may_load().map(|opt| opt.unwrap_or_default())
}