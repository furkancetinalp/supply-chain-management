// Aylık olarak alınacak tahmini sipariş/satım sayısı gir (ay ve yıl input alınabilir,
// ürün ismi alınabilir, tek ürün üretiliyorsa yine de isim alınsa daha iyi olur, neyin tahmini çünkü)

// Yıllık olarak alınacak tahmini sipariş/satım sayısı gir (yıl ve üretilecek ürün ismi parametre olarak alınabilir)
// Girilen tahmini güncelle
// Girilen tahmini sil
// Kaydedilen tahminleri ekrana getir

//DEMAND FORECASTING ADINDA BİR SAYFA YAPILABİLİR, BURADA GELECEKLE İLGİLİ ANALİZLER, İHTİYAÇLAR, TALEP TAHMİNLERİ GİRİLİR



//1. Demand planning
//2. Supply Planning 
//3. Raw Material Planning
//3. Production planning
// use chrono::{DateTime, TimeZone, Utc};
// use chrono::prelude::*;
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


#[derive(CandidType, Deserialize, Clone)]
struct DemandPlan{
    identity:String,
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

#[derive(CandidType, Deserialize, Clone)]
struct Time{
    year:u16,
    month:u8,
    date:u8,
    hour:u8,
    minute:u8,
    formatted:String,

}
// //memory definition
// type Memory = VirtualMemory<DefaultMemoryImpl>;
// const MAX_VALUE_SIZE: u32 = 1024;

// impl Storable for DemandPlan {
//     fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
//         Cow::Owned(Encode!(self).unwrap())
//     }

//     fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
//         Decode!(bytes.as_ref(), Self).unwrap()
//     }
// }

// impl BoundedStorable for DemandPlan {
//     const MAX_SIZE: u32 = MAX_VALUE_SIZE;
//     const IS_FIXED_SIZE: bool = false;
// }



// Creating memory manager with a new MemoryId
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
    RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static DEMAND_PLAN_MAP: RefCell<HashMap<u32, DemandPlan>> = RefCell::default();

}



//ADD DEMAND
#[ic_cdk::update]
 pub async fn add_demand_plan(mut demand: DemandPlan) -> Option<bool> {
    let created_date = ic_cdk::api::time().to_string();
    
    let unique_id:u32 = idgenerator::create_id().await;
    demand.id = unique_id;
    demand.created_date=created_date;
    let user_id = ic_cdk::caller();
    demand.identity=user_id.to_string();
    DEMAND_PLAN_MAP.with(|p| p.borrow_mut().insert(unique_id, demand));
    return Some(true);
}

//GET ALL
#[ic_cdk::query]
pub fn get_all_demand_plans() -> Vec< DemandPlan> {
    let data: Vec<DemandPlan> = DEMAND_PLAN_MAP.with(|demands| {
        let binding = demands.borrow();
        let filter = binding.iter().filter(|& x| x.1.identity == ic_cdk::caller().to_string()).collect::<Vec<_>>();
        let result = filter.iter().map(|x| x.1.clone()).collect::<Vec<_>>();
        return result;
    });

    return data;
    // let data:Vec<_> = DEMAND_PLAN_MAP.with(|p| p.borrow().iter()
    // .filter(|& x| x.1.identity == ic_cdk::api::caller().to_string()).collect::<Vec<_>>());
    // // .collect::<Vec<_>>();

    // let result = data.iter().map(|x| x.1.clone()).collect::<Vec<_>>();
    // return result;
    // let response:Vec<_> =   DEMAND_PLAN_MAP.with(|p| p.borrow().iter().map(|x: (u32, DemandPlan)| x.1).collect());
}

//UPDATE
#[ic_cdk::update]
 pub async fn update_demand_plan(mut demand: DemandPlan) -> Option<bool> {
    let created_date = ic_cdk::api::time().to_string();
    
    demand.created_date=created_date;
    let user_id = ic_cdk::caller();
    demand.identity=user_id.to_string();
    DEMAND_PLAN_MAP.with(|p| p.borrow_mut().insert(demand.id, demand));
    return Some(true);
}


#[ic_cdk::query]
pub fn get_demand_plans_by_name(name:String) -> Vec< DemandPlan> {
    let data: Vec<DemandPlan> = DEMAND_PLAN_MAP.with(|demands| {
        let binding = demands.borrow();
        let filter = binding.iter()
        .filter(|& x| x.1.identity == ic_cdk::caller().to_string() && x.1.name.to_lowercase()==name.to_lowercase()).collect::<Vec<_>>();
        let result = filter.iter().map(|x| x.1.clone()).collect::<Vec<_>>();
        return result;
    });
    return data;
}

#[ic_cdk::query]
pub fn get_demand_plans_by_customer_group(customer_group:String) -> Vec< DemandPlan> {
    let data: Vec<DemandPlan> = DEMAND_PLAN_MAP.with(|demands| {
        let binding = demands.borrow();

        let filter = binding.iter()
        .filter(|& x| x.1.identity == ic_cdk::caller().to_string() && 
                    x.1.customer_group.to_lowercase()==customer_group.to_lowercase()).collect::<Vec<_>>();

        let result = filter.iter().map(|x| x.1.clone()).collect::<Vec<_>>();
        return result;
    });
    return data;
}

#[ic_cdk::query]
pub fn get_demand_plans_by_year_range(from:u16,to:u16) -> Vec< DemandPlan> {
    let data: Vec<DemandPlan> = DEMAND_PLAN_MAP.with(|demands| {
        let binding = demands.borrow();
        
        let filter = binding.iter()
        .filter(|& x| x.1.identity == ic_cdk::caller().to_string() && 
                    x.1.from.year>=from && x.1.to.year<=to).collect::<Vec<_>>();

        let result = filter.iter().map(|x| x.1.clone()).collect::<Vec<_>>();
        return result;
    });
    return data;
}

#[ic_cdk::query]
pub fn delete_demand_plan(_id:u32) -> bool {
    let result = DEMAND_PLAN_MAP.with(|p| p.borrow_mut().to_owned().remove_entry(&_id));
    match result {
        Some(data) =>true,
        None => false
    }
}



#[derive(CandidType, Deserialize, Clone)]
enum Unit{
    Kg,
    Tonne,
    Piece,

}
enum UrgencyLevel{
    Lowest,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(CandidType, Deserialize, Clone)]
enum Period{
    Q1,
    Q2,
    Q3,
    Q4,
    Yearly,
    Monthly,
    Daily,
    Weekly,
}
pub enum Month {
    January = 0,
    February = 1,
    March = 2,
    April = 3,
    May = 4,
    June = 5,
    July = 6,
    August = 7,
    September = 8,
    October = 9,
    November = 10,
    December = 11,
}




// let utc = Utc::now().to_owned().format("%d/%m/%Y %H:%M").to_string();

// #[ic_cdk::query]
// pub fn get_all_demand_plans2() -> Vec< DemandPlan> {
//     let data: Vec<DemandPlan> = DEMAND_PLAN_MAP.with(|demands| {
//         let binding = demands.borrow();
//         // let filter = binding.iter().filter(|& x| x.1.identity == ic_cdk::api::caller().to_string()).collect::<Vec<_>>();
//         let result = binding.iter().map(|x| x.1.clone()).collect::<Vec<_>>();
//         return result;
//     });

//     return data;
//     // let data:Vec<_> = DEMAND_PLAN_MAP.with(|p| p.borrow().iter()
//     // .filter(|& x| x.1.identity == ic_cdk::api::caller().to_string()).collect::<Vec<_>>());
//     // // .collect::<Vec<_>>();

//     // let result = data.iter().map(|x| x.1.clone()).collect::<Vec<_>>();
//     // return result;
//     // let response:Vec<_> =   DEMAND_PLAN_MAP.with(|p| p.borrow().iter().map(|x: (u32, DemandPlan)| x.1).collect());
// }
