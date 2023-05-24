#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Addr,Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::state::{SETUP,Setup,POLL,Poll};
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};


const CONTRACT_NAME: &str = "crates.io:counter";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
 

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {

set_contract_version(_deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;


//validating the instantiator address
    let validated_admin :Addr = _deps.api.addr_validate(&_msg.admin_address)?;
    let setup = Setup {
        owner : validated_admin,
    };

    SETUP.save(_deps.storage, &setup)?;
    Ok(Response::new().add_attribute("action", "instantiate"))

    
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result <Response, ContractError> {
    match msg {
      ExecuteMsg::CreatePoll { question } => execute_create_poll(deps, env, info, question),
      ExecuteMsg::Vote { question, choice } => execute_vote(deps, env, info, question,choice),
    }
}
   
    fn execute_create_poll(deps: DepsMut,_env: Env,_info : MessageInfo,question : String) -> Result<Response, ContractError> { 

//checking duplication of key for the poll message 
        if POLL.has(deps.storage,question.clone()) {
            return Err(ContractError::CustomError { val: "key already taken".to_string() });
        }

        
      let poll= Poll {
        question: question.clone(),
        yes : 0,
        no:0,
      };

      POLL.save(deps.storage,question,&poll)?;
      Ok(Response::new().add_attribute("action","created_poll"))
    }
   





pub fn execute_vote(deps:DepsMut, env:Env, info:MessageInfo, question:String,choice:String) -> Result<Response, ContractError> {
    unimplemented!();

    //check if poll hasn't been created yet for that message(key)
    
    if !POLL.has(deps.storage,question.clone()) {
        return Err(ContractError::CustomError { val: "Poll doesn't exist".to_string() });
    }

    //extract poll state of the question from mapping
    let mut poll = POLL.load(deps.storage,question.clone())?;

    //check the choice and update the state
    
    if choice == "yes"{
        poll.yes += 1;
    }else if choice == "no" {
        poll.no += 1;
    }else {
        return Err(ContractError::CustomError { val: "Invalid choice".to_string()})
    }


    }
  





#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
 //to mock the all required values
 use cosmwasm_std::{attr, Response};
 use cosmwasm_std::testing::{mock_dependencies,mock_env,mock_info};
 use crate::contract::{execute,instantiate};
 use crate::msg::{InstantiateMsg, ExecuteMsg};
#[test]
   

    fn test_instantiate() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("addr1", &[]);


        //define the msg
        let msg = InstantiateMsg {
            admin_address: String::from("addr1"),

        };

        let resp = instantiate(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(resp.attributes,vec![attr("action", "instantiate")]);

    }

    #[test]

    fn test_create_poll() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("addr1", &[]);

        let msg = InstantiateMsg {
            admin_address: String::from("addr1"),

        };

//we have to instantiate the contract before executing it
let _resp = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();


        //define the msg
     let msg = ExecuteMsg::CreatePoll { question: "Do you like ElonMusk ?".to_string() };

    let resp = execute(deps.as_mut(),env,info,msg).unwrap();
        assert_eq!(resp.attributes,vec![attr("action", "created_poll")]);
    }

   
        


}

