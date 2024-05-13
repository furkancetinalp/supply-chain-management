use candid::{CandidType, Decode, Deserialize, Encode, Principal};

#[derive(CandidType, Deserialize, Clone)]
pub struct Time{
    pub year:u16,
    pub month:u8,
    pub date:u8,
    pub hour:u8,
    pub minute:u8,
    pub formatted:String,

}