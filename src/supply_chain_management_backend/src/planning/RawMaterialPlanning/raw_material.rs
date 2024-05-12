
#[derive(CandidType, Deserialize, Clone)]
struct RawMaterial{
    identity:String, //identity of company/user
    id:u32,
    name:String,//product name
    description:String, //product description
    customer_group:String, //corporate,foundation,government,hospitality,education,individual
    amount: u64,
    unit: Unit,
    created_date:String,
    from:Time,
    to:Time,
    // target_year:String,
    // period:Period,

}