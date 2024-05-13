use candid::{CandidType, Decode, Deserialize, Encode, Principal};

use crate::entities::{time::Time, unit::Unit, urgency::Urgency};

#[derive(CandidType, Deserialize, Clone)]
pub struct DemandPlan{
    pub identity:String,
    pub id:u32,
    pub name:String,//product name
    pub description:String, //product description
    pub customer_group:String, //corporate,foundation,government,hospitality,education,individual
    pub amount: u64,
    pub unit: Unit,
    pub created_date:String,
    pub from:Time,
    pub to:Time,
    // target_year:String,
    // period:Period,

}