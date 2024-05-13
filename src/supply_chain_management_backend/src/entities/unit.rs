use candid::{CandidType, Decode, Deserialize, Encode, Principal};

#[derive(CandidType, Deserialize, Clone)]
pub enum Unit{
    Kg,
    Tonne,
    Piece,

}