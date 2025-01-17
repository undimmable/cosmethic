use crate::msg::{Action, ExecuteMsg, InstantiateMsg, QueryAction, QueryMsg, Votes};
use cosmwasm_std::StdError;
use cosmwasm_std::{
    entry_point, from_binary, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response,
    StdResult,
};
use std::fmt::Binary;

pub const MAX_BUDGET: u128 = 1000;

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
        //why did you add voting in storage? voting in latch?
        ExecuteMsg::Vote { proposal_id, vote } => vote(deps, info, proposal_id, vote),
        // Action::SubmitVote { scenario, budget } => submit_vote(deps, info, scenario, budget),
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg.action {
        QueryAction::GetVotes {} => to_binary(&get_votes(deps)),
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

fn get_votes(deps: Deps) -> Vec<u8> {
    deps.storage.get(b"votes").unwrap_or_default()
}


#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        from_binary,
        testing::{mock_dependencies, mock_env, mock_info},
    };
    use crate::democracy::{execute, instantiate, query};
    use crate::msg::{ExecuteMsg, InstantiateMsg, QueryAction, QueryMsg, Votes};

    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {
            scenarios: vec!["Scenario 1".to_string(), "Scenario 2".to_string()],
        };
        let info = mock_info("creator", &[]);

        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(res.attributes[0].value, "instantiate");

        let votes = load_votes(&deps.storage).unwrap();
        assert_eq!(votes.scenario_votes.len(), 2);
        assert_eq!(votes.scenario_votes[0], ("Scenario 1".to_string(), 0));
    }

    #[test]
    fn test_submit_vote() {
        let mut deps = mock_dependencies();

        // Инициализация
        let msg = InstantiateMsg {
            scenarios: vec!["Scenario 1".to_string(), "Scenario 2".to_string()],
        };
        let info = mock_info("creator", &[]);
        instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // Успешное голосование
        let vote_msg = ExecuteMsg {
            action: Action::SubmitVote {
                scenario: "Scenario 1".to_string(),
                budget: 100,
            },
        };
        let info = mock_info("voter1", &[]);
        let res = execute(deps.as_mut(), mock_env(), info, vote_msg).unwrap();
        assert_eq!(res.attributes[0].value, "submit_vote");

        let votes = load_votes(&deps.storage).unwrap();
        assert_eq!(votes.scenario_votes[0], ("Scenario 1".to_string(), 100));

        // Ошибка: превышение бюджета
        let vote_msg = ExecuteMsg {
            action: Action::SubmitVote {
                scenario: "Scenario 2".to_string(),
                budget: 1001,
            },
        };
        let info = mock_info("voter2", &[]);
        let err = execute(deps.as_mut(), mock_env(), info, vote_msg).unwrap_err();
        assert_eq!(err.to_string(), ERR_BUDGET_EXCEEDED);

        // Ошибка: голос за несуществующий сценарий
        let vote_msg = ExecuteMsg {
            action: Action::SubmitVote {
                scenario: "Invalid".to_string(),
                budget: 50,
            },
        };
        let err = execute(deps.as_mut(), mock_env(), info, vote_msg).unwrap_err();
        assert_eq!(err.to_string(), ERR_SCENARIO_NOT_FOUND);
    }

    #[test]
    fn test_query_votes() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {
            scenarios: vec!["Scenario 1".to_string(), "Scenario 2".to_string()],
        };
        let info = mock_info("creator", &[]);
        instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        let query_msg = QueryMsg {
            action: QueryAction::GetVotes,
        };
        let bin = query(deps.as_ref(), mock_env(), query_msg).unwrap();
        let votes: Votes = from_binary(&bin).unwrap();

        assert_eq!(votes.scenario_votes.len(), 2);
        assert_eq!(votes.scenario_votes[0], ("Scenario 1".to_string(), 0));
    }
}