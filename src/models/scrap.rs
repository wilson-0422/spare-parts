use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Scrap {
    pub id: i64,
    pub material_id: i64,
    pub quantity: f64,
    pub reason: String,
    pub handler: String,
    pub status: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScrapWithMaterial {
    pub scrap: Scrap,
    pub material_name: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateScrapForm {
    pub material_id: i64,
    pub quantity: f64,
    pub reason: String,
    pub handler: String,
}
