use candid::{CandidType, Deserialize};
use crate::entities::{time::Time, unit::Unit};

#[derive(CandidType, Deserialize, Clone)]
pub struct Demand{
    pub id:u32,
    pub identity:String,
    pub name:String,//product name
    pub description:String, //product description
    pub customer_group:String, //corporate,foundation,government,hospitality,education,individual
    pub amount: f64,
    pub unit: Unit,
    pub created_date:String,
    pub from:Time,
    pub to:Time,
    // target_year:String,
    // period:Period,

}


#[derive(CandidType, Deserialize, Clone)]
pub struct DemandRequest{
    pub id:u32,
    pub identity:String,
    pub name:String,//product name
    pub description:String, //product description
    pub customer_group:String, //corporate,foundation,government,hospitality,education,individual
    pub amount: f64,
    pub unit: Unit,
    pub created_date:String,
    pub from:Time,
    pub to:Time,
    // target_year:String,
    // period:Period,

}