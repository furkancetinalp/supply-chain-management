
use candid::{CandidType, Deserialize, Principal};
use crate::entities::{time::Time, unit::Unit, urgency::Urgency};

#[derive(CandidType, Deserialize, Clone)]
pub(crate) struct RawMaterial{
    pub id:u32,
    pub identity:String,
    pub name:String,
    pub description:String,
    pub amount: f64,
    pub unit: Unit,
    pub from:Time,
    pub to:Time,
    pub urgency:Urgency,
    pub warehouse_name:String, //warehouseId olarak değişmeli
    pub requested_delivery_time:u8, //required date to be delivered
    pub created_date:String,

}


#[derive(CandidType, Deserialize, Clone)]
pub(crate) struct RawMaterialRequest{
    // pub id:u32,
    // pub identity:String,
    pub name:String,
    pub description:String,
    pub amount: f64,
    pub unit: Unit,
    pub from:Time,
    pub to:Time,
    pub urgency:Urgency,
    pub warehouse_name:String, //warehouseId olarak değişmeli
    pub requested_delivery_time:u8, //required date to be delivered
    // pub created_date:String,

}