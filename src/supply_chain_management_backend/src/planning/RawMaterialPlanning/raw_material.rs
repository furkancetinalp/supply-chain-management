
use candid::{CandidType, Decode, Deserialize, Encode, Principal};

use crate::entities::{time::Time, unit::Unit, urgency::Urgency};


#[derive(CandidType, Deserialize, Clone)]
struct RawMaterial{
    identity:Principal,
    id:u32,

    name:String,
    description:String,
    amount: f64,
    unit: Unit,

    from:Time,
    to:Time,
    urgency:Urgency,
    warehouse_name:String,

    created_date:String,

}
