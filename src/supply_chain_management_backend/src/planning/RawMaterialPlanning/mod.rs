mod raw_material;
use crate::entities;

use ic_cdk::call;
use std::borrow::{BorrowMut, Cow};
use std::cell::RefCell;
use std::collections::HashMap;

use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, DefaultMemoryImpl, StableBTreeMap, Storable};
use serde::Serialize;

use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod,
};
use rand::{CryptoRng, RngCore, SeedableRng};
use super::idgenerator;





// Get a random number generator based on 'raw_rand'.
// Based on https://github.com/dfinity/internet-identity/blob/f76e36cc45e064b5e04b977de43698c13e7b55d9/src/internet_identity/src/main.rs#L683-L697
