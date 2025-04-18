use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ExecuteMsg {
    pub action: Action,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Action {
    SubmitVote { scenario: String, budget: u128 }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct QueryMsg {
    pub action: QueryAction,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum QueryAction {
    GetVotes {}
}
