use cosmwasm_std::{to_json_binary, entry_point, DepsMut, Empty, Env, MessageInfo, Response, StdError, WasmMsg};
use crate::msg::{ExecuteMsg, ExecuteMsg::SetOther, ExecuteMsg::PingPong, InstantiateMsg};
use crate::state::OTHER;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, StdError> {

    match msg {
        ExecuteMsg::PingPong() => {
            let other = OTHER.load(deps.storage)?;
            let ping_pong_msg = ExecuteMsg::PingPong {  };
            let exec = WasmMsg::Execute {
                contract_addr: other,
                msg: to_json_binary(&ping_pong_msg)?,
                funds: vec![]
            };
            return Ok(Response::new().add_message(exec));
        },
        ExecuteMsg::SetOther(o) => {
            OTHER.save(deps.storage, &o.other)?;
            return Ok(Response::new());
        }
    }
   
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, StdError> {

    OTHER.save(deps.storage, &msg.other)?;
    return Ok(Response::new());
   
}

#[cfg(test)]
mod test {
    use cosmwasm_std::{testing::{mock_dependencies, mock_env, mock_info}, to_json_binary, CosmosMsg, Response, WasmMsg};
    use crate::msg::InstantiateMsg;
    use crate::msg::ExecuteMsg;

    #[test]
    fn ping_pong_works() {
        let mut deps = mock_dependencies();
        let instantiate_msg = InstantiateMsg { other: "other".to_string() };
        let msg = ExecuteMsg::PingPong {  };
        crate::contract::instantiate(deps.as_mut(), mock_env(), mock_info("creator", &[]), instantiate_msg).unwrap();
        let response = crate::contract::execute(deps.as_mut(), mock_env(), mock_info("creator", &[]), msg).unwrap();
        assert_eq!(response.messages.len(), 1);
        let expected_msg = WasmMsg::Execute { contract_addr: String::from("other"), msg: to_json_binary(&ExecuteMsg::PingPong{}).unwrap(), funds: vec![] }.into();
        assert_eq!(
            response.messages[0].msg,
            expected_msg
        );
    }

    #[test]
    fn set_other_works() {
        let mut deps = mock_dependencies();
        let instantiate_msg = InstantiateMsg { other: "other".to_string() };
        let msg = ExecuteMsg::SetOther( InstantiateMsg { other: "other2".to_string() });
        crate::contract::instantiate(deps.as_mut(), mock_env(), mock_info("creator", &[]), instantiate_msg).unwrap();
        let response = crate::contract::execute(deps.as_mut(), mock_env(), mock_info("creator", &[]), msg).unwrap();
        assert_eq!(response.messages.len(), 0);
    }
}