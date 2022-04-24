#[cfg(not(feature = "library"))]
use cosmwasm_std::Addr;
use cosmwasm_std::entry_point;
//[CHANGE] delete some un-needed dependencies
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use cw2::set_contract_version;

use crate::error::ContractError;
//[CHANGE] update imports from msg
use crate::msg::{ExecuteMsg, InstantiateMsg, GameMove, GameResult};
//[CHANGE] update state imports
use crate::state::{State, STATE, Data, ENTRIES};
//[CHANGE] imports enums from msg


// version info for migration info
const CONTRACT_NAME: &str = "crates.io:counter";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        count: msg.count,
        owner: info.sender.clone(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("count", msg.count.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::StartGame { opponent, current_move } => try_start(deps, info, opponent, current_move),
    }
}

pub fn try_start(
    deps: DepsMut,
    info: MessageInfo,
    opponent: Addr,
    current_move: GameMove,
) -> Result<Response, ContractError> {
    //[CHANGE] had the function continue if the match isn't an error
    match deps.api.addr_validate(opponent.as_str()) {
        Err(_) =>  return Err(ContractError::Unauthorized {}),
        Ok(_) =>  {},
    };
    
    //[CHANGE] save the shit, see docs: https://github.com/CosmWasm/cw-plus/tree/main/packages/storage-plus
    // let state = STATE.load(deps.storage)?;
    let data = Data {opponent:opponent, host_move: current_move, opp_move: GameMove::None, result: GameResult::Pending};
    ENTRIES.save(deps.storage, info.sender, &data)?;
    Ok(Response::new().add_attribute("method", "try_start"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    //[CHANGE] delete some un-needed dependencies
    use cosmwasm_std::{coins};

    #[test]
    fn start_test_rock() {
        //[CHANGE] you should prolly know this stuff by now.
        let mut deps = mock_dependencies(&coins(2, "token"));
        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        
        let info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::StartGame {
            opponent: Addr::unchecked("terra1j5tyxu0d7tejjqe9u9d5vvme4ngqucwlgdx4v0"),
            current_move: GameMove::Rock,
        };
        
        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
        println!("response: {:?}", res);
        //response: Response { messages: [], attributes: [Attribute { key: "method", value: "try_start" }], events: [], data: None }
        assert_eq!("try_start", res.attributes[0].value);
    }
}
