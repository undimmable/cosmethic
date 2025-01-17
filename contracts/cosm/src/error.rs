use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),
    #[error("You think you're so smart, don't ya?")]
    NotSapientEnough {},
    #[error("Unauthorized. This is not the cyborg you're looking for.")]
    Unauthorized {},
}
