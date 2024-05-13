use candid::{CandidType, Decode, Deserialize, Encode, Principal};

#[derive(CandidType, Deserialize, Clone)]

pub enum Urgency{
    Lowest,
    Low,
    Medium,
    High,
    Critical,
}