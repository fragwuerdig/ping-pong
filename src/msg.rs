use cosmwasm_schema::cw_serde;
use cosmwasm_std::Empty;


#[cw_serde]
pub struct InstantiateMsg {
    pub other: String
}

#[cw_serde]
pub enum ExecuteMsg {
    PingPong{},
    SetOther(InstantiateMsg)
}