use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct HiMsg {
    pub name: String,
    pub message: String,
}

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    SayHi(HiMsg),
}
