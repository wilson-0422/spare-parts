use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Requisition {
    pub id: i64,
    pub material_id: i64,
    pub quantity: f64,
    pub department: String,
    pub applicant: String,
    pub purpose: String,
    pub status: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RequisitionWithMaterial {
    pub requisition: Requisition,
    pub material_name: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateRequisitionForm {
    pub material_id: i64,
    pub quantity: f64,
    pub department: String,
    pub applicant: String,
    pub purpose: String,
}
