use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdResult};

pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: (),
) -> StdResult<Response> {
    Ok(Response::default())
}
