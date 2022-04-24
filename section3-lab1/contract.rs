#[cfg(not(feature = "library"))]
use cosmwasm_std::Addr;
use cosmwasm_std::entry_point;
//[CHANGE] delete some un-needed dependencies
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use cw2::set_contract_version;

use crate::error::ContractError;
//[CHANGE] update imports
//[CHANGE] delete some un-needed dependencies
use crate::msg::{ExecuteMsg, InstantiateMsg};
use crate::state::{State, STATE};

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
        ExecuteMsg::StartGame { opponent } => try_start(deps, info, opponent),
    }
}

//[CHANGE] created try_start regular with same arguments and return value from section 1
pub fn try_start(
    deps: DepsMut,
    _info: MessageInfo,
    opponent: Addr,
) -> Result<Response, ContractError> {
    //[CHANGE] validate the addr (step 3) https://docs.rs/cosmwasm-std/0.14.0/cosmwasm_std/struct.Addr.html
    //used pattern matching to validate address and return an error or ok depending on the value.
    return match deps.api.addr_validate(opponent.as_str()) {
        Ok(_) =>  Ok(Response::new().add_attribute("method", "try_start")),
        Err(_) =>  Err(ContractError::Unauthorized {}),
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    //[CHANGE] delete some un-needed dependencies
    use cosmwasm_std::{coins};

    #[test]
    fn start_test() {
        //[CHANGE] you should prolly know this stuff by now.
        let mut deps = mock_dependencies(&coins(2, "token"));
        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        
        let info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::StartGame {
            opponent: Addr::unchecked("terra1j5tyxu0d7tejjqe9u9d5vvme4ngqucwlgdx4v0"),
        };
        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
        println!("response: {:?}", res);
        //response: Response { messages: [], attributes: [Attribute { key: "method", value: "try_start" }], events: [], data: None }
        assert_eq!("try_start", res.attributes[0].value);
    }
    #[test]
    #[should_panic]
    fn wrong_address_test() {
        let mut deps = mock_dependencies(&coins(2, "token"));
        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        
        let info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::StartGame {
            opponent: Addr::unchecked("jo"),
        };
        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!("try_start", res.attributes[0].value);
    }


}
