use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct InstantiateMsg {
    pub scenarios: Vec<String>, // список сценариев
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ExecuteMsg {
    pub action: Action,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Action {
    SubmitVote { scenario: String, budget: u128 }, // голосование
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct QueryMsg {
    pub action: QueryAction,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum QueryAction {
    GetVotes {}, // query votes
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct Votes {
    pub scenario_votes: Vec<(String, u128)>, // scenarios and budgets list
}