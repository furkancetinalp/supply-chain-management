pub(crate) mod raw_material;
use crate::context::RAW_MATERIAL_PLAN;
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
use self::raw_material::RawMaterial;

use super::idgenerator;



#[ic_cdk::update]
 pub async fn add_raw_material_plan(mut request: RawMaterial) -> Option<bool> {
    let created_date = ic_cdk::api::time().to_string();
    
    let unique_id:u32 = idgenerator::create_id().await;
    request.id = unique_id;
    request.created_date=created_date;
    let user_id = ic_cdk::caller();
    request.identity=user_id.to_string();
    RAW_MATERIAL_PLAN.with(|p| p.borrow_mut().insert(unique_id, request));
    return Some(true);
}



//GET ALL
#[ic_cdk::query]
pub fn get_all_raw_material_plans() -> Vec< RawMaterial> {
    let data: Vec<RawMaterial> = RAW_MATERIAL_PLAN.with(|demands| {
        let binding = demands.borrow();
        let filter = binding.iter().filter(|& x| x.1.identity == ic_cdk::caller().to_string()).collect::<Vec<_>>();
        let result = filter.iter().map(|x| x.1.clone()).collect::<Vec<_>>();
        return result;
    });
    return data;
}

//UPDATE
#[ic_cdk::update]
 pub async fn update_raw_material_plan(mut demand: RawMaterial) -> Option<bool> {
    let created_date = ic_cdk::api::time().to_string();
    
    demand.created_date=created_date;
    let user_id = ic_cdk::caller();
    demand.identity=user_id.to_string();
    RAW_MATERIAL_PLAN.with(|p| p.borrow_mut().insert(demand.id, demand));
    return Some(true);
}

