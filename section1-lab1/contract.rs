#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

//[CHANGE] Added Addr type to pass to the change owner function
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Addr};
use cw2::set_contract_version;

use crate::error::ContractError;
//[CHANGE] The new UserResponse struct is imported.
use crate::msg::{CountResponse, UserResponse, ExecuteMsg, InstantiateMsg, QueryMsg}; 
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
        ExecuteMsg::Increment {} => try_increment(deps),
        ExecuteMsg::Reset { count } => try_reset(deps, info, count),

        //[CHANGE] Adding the ChangeOwner function to this match statement to call the try_change_owner function
        //recall ChangeOwner is listed in the execute msg section on line 19 in msg.rs
        ExecuteMsg::ChangeOwner { address } => try_change_owner(deps, info, address),
    }
}

 //[CHANGE] Adding a function to actually change the owner
 //first look at the arguments, deps holds data from the cosmwasm std library which is imported at the top of this page (line 5)
 //info holds a variable of the MessageInfo type which is also imported at top of this page (line 5) this hold request data
 //address holds a variable of the Addr type which is used for setting the new owner's address
pub fn try_change_owner(deps: DepsMut,
    info: MessageInfo,
    address: Addr) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {

         //[CHANGE] check to make sure the sender is the same as the current owner of the coutner
        if info.sender != state.owner {
            return Err(ContractError::Unauthorized {});
        }
        //[CHANGE] set the new Owner
        state.owner = address;
        Ok(state)
    })?;

    Ok(Response::new().add_attribute("method", "try_change_owner"))
}

pub fn try_increment(deps: DepsMut) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        state.count += 1;
        Ok(state)
    })?;

    Ok(Response::new().add_attribute("method", "try_increment"))
}
pub fn try_reset(deps: DepsMut, info: MessageInfo, count: i32) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        if info.sender != state.owner {
            return Err(ContractError::Unauthorized {});
        }
        state.count = count;
        Ok(state)
    })?;
    Ok(Response::new().add_attribute("method", "reset"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCount {} => to_binary(&query_count(deps)?),
        
        //[CHANGE] Adding the function we defined in msg.rs line 29 to this match (basically a 
        //struct https://doc.rust-lang.org/rust-by-example/flow_control/match.html)
        //this calls a new function "query_user" which is defined below
        QueryMsg::GetUser {} => to_binary(&query_user(deps)?),
    }
}

//[CHANGE] New function to get the user data. Notice i put StdResult<UserResponse> User response
//being the struct i defined in msg.rs line 40
fn query_user(deps: Deps) -> StdResult<UserResponse> {
    let state = STATE.load(deps.storage)?;
     //[CHANGE] state.owner is using the state struct provided on line 24 of this page.
     //notice hose the first part of this, "{ owner:" is that way because i defined UserResponse that way
     //in msg.rs line 40.
    Ok(UserResponse { owner: state.owner })
}

fn query_count(deps: Deps) -> StdResult<CountResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(CountResponse { count: state.count })
}



#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(&[]);

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(17, value.count);
    }

    #[test]
    fn increment() {
        let mut deps = mock_dependencies(&coins(2, "token"));

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Increment {};
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // should increase counter by 1
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(18, value.count);
    }

    #[test]
    fn query_owner_test() {
        //[CHANGE] initialize stuff. look at line 18 and how "pub fn instantiate" works to get context on this
        let mut deps = mock_dependencies(&coins(2, "token"));
        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(2, "token"));

        //[CHANGE] Calling the instantiate function
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        
        //[CHANGE] After instantiate happens we do a "query" function which is defined on line 95.
        //we are basically passing the stuff we initialized in the past few lines as arguments
        //into this function.
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetUser {}).unwrap();

        //[CHANGE] Deserialize the response (decipher it)
        let value: UserResponse = from_binary(&res).unwrap();

        //[CHANGE] the test will pass if the fake info we provided: creator is the same as
        //the value we get back
        assert_eq!(String::from("creator"), value.owner);
    }


    #[test]
    fn change_owner_test() {
        //[CHANGE] create data for initializing the contract, line 195 inits it with this data.
        let mut deps = mock_dependencies(&coins(2, "token"));
        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();


        //[CHANGE] re-initialize stuff for executing the ChangeOwner function
        let info = mock_info("creator", &coins(2, "token"));

        //[CHANGE] Creating a new msg that will have our fake new address
        let msg = ExecuteMsg::ChangeOwner {
            address: Addr::unchecked("a new user address :0"),
        };
        
        //[CHANGE] execute the change owner function
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        //[CHANGE] check to see if the owner was changed
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetUser {}).unwrap();
        let value: UserResponse = from_binary(&res).unwrap();
        assert_eq!(String::from("a new user address :0"), value.owner);
    }
    #[test]
    fn reset() {
        let mut deps = mock_dependencies(&coins(2, "token"));

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let unauth_info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Reset { count: 5 };
        let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
        match res {
            Err(ContractError::Unauthorized {}) => {}
            _ => panic!("Must return unauthorized error"),
        }

        // only the original creator can reset the counter
        let auth_info = mock_info("creator", &coins(2, "token"));
        let msg = ExecuteMsg::Reset { count: 5 };
        let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

        // should now be 5
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(5, value.count);
    }
}
