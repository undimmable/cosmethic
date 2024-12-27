use cosmwasm_std::StdError;
use cosmwasm_std::{
    entry_point, from_binary, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response,
    StdResult,
};
use serde::{Deserialize, Serialize};

pub const MAX_BUDGET: u128 = 1000;

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

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let votes = Votes {
        scenario_votes: msg.scenarios.into_iter().map(|s| (s, 0)).collect(),
    };

    deps.storage.set(b"votes", &to_binary(&votes)?);
    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg.action {
        Action::SubmitVote { scenario, budget } => submit_vote(deps, info, scenario, budget),
    }
}

fn submit_vote(
    deps: DepsMut,
    _info: MessageInfo,
    scenario: String,
    budget: u128,
) -> StdResult<Response> {
    let mut votes: Votes = deps
        .storage
        .get(b"votes")
        .unwrap_or_default()
        .map(|data| from_binary(&data).unwrap());

    for (s, b) in votes.scenario_votes.iter_mut() {
        if *s == scenario {
            // Check that the total budget does not exceed the maximum allowed amount
            let total_budget: u128 = votes.scenario_votes.iter().map(|(_, b)| *b).sum();

            if let Some(value) = validate(&scenario, budget, &mut votes, total_budget) {
                return value;
            }
            *b += budget;
        }
    }
    deps.storage.set(b"votes", &to_binary(&votes)?);
    Ok(Response::default())
}

fn validate(scenario: &String, budget: u128, mut votes: &mut Votes, total_budget: u128) -> Option<StdResult<Response>> {
    if total_budget + budget > MAX_BUDGET {
        return Some(Err(StdError::generic_err(
            "Total budget exceeds the maximum allowed amount",
        )));
    }

    if let Some(value) = check_has_budget(budget) {
        return Some(value);
    }

    if let Some(value) = check_scenario_exists(&scenario, &mut votes) {
        return Some(value);
    }

    if let Some(value) = check_has_not_voted_for_scenario(&scenario, &mut votes) {
        return Some(value);
    }

    if let Some(value) = check_has_not_voted_for_other_scenarios(&scenario, &mut votes) {
        return Some(value);
    }
    None
}

fn check_has_not_voted_for_other_scenarios(
    scenario: &String,
    votes: &mut Votes,
) -> Option<StdResult<Response>> {
    // Check that the user has not voted for other scenarios
    if votes
        .scenario_votes
        .iter()
        .any(|(s, b)| s != scenario && *b > 0)
    {
        Some(Err(StdError::generic_err(
            "User has already voted for another scenario",
        )))
    } else {
        None
    }
}

fn check_has_not_voted_for_scenario(
    scenario: &String,
    votes: &mut Votes,
) -> Option<StdResult<Response>> {
    // Check that the user has not already voted for this scenario
    if votes
        .scenario_votes
        .iter()
        .any(|(s, b)| s == scenario && *b > 0)
    {
        return Some(Err(StdError::generic_err(
            "User has already voted for this scenario",
        )));
    }
    None
}

fn check_scenario_exists(scenario: &String, votes: &Votes) -> Option<StdResult<Response>> {
    if !votes.scenario_votes.iter().any(|(s, _)| s == scenario) {
        return Some(Err(StdError::generic_err("Scenario does not exist")));
    }
    None
}

fn check_has_budget(budget: u128) -> Option<StdResult<Response>> {
    if budget <= 0 {
        return Some(Err(StdError::generic_err(
            "Budget must be greater than zero",
        )));
    }
    None
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg.action {
        QueryAction::GetVotes {} => to_binary(&get_votes(deps)),
    }
}

fn get_votes(deps: Deps) -> Vec<u8> {
    deps.storage.get(b"votes").unwrap_or_default()
}