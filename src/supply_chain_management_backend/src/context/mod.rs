use std::cell::RefCell;
use std::collections::HashMap;
use crate::planning::DemandPlanning::demand::Demand;
use crate::planning::RawMaterialPlanning::raw_material::RawMaterial;

thread_local! {
    pub static DEMAND_PLAN_MAP: RefCell<HashMap<u32, Demand>> = RefCell::default();
    pub static RAW_MATERIAL_PLAN: RefCell<HashMap<u32, RawMaterial>> = RefCell::default();

}
