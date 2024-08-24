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
        PingPong => {
            let other = OTHER.load(deps.storage)?;
            let ping_pong_msg = ExecuteMsg::PingPong {  };
            let exec = WasmMsg::Execute {
                contract_addr: other,
                msg: to_json_binary(&ping_pong_msg)?,
                funds: vec![]
            };
            return Ok(Response::new().add_message(exec));
        },
        SetOther(o) => {
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