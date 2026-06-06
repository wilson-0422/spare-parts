use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Material {
    pub id: i64,
    pub name: String,
    pub category: String,
    pub specification: String,
    pub unit: String,
    pub stock_quantity: f64,
    pub min_stock: f64,
    pub price: f64,
    pub supplier_id: Option<i64>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MaterialWithSupplier {
    pub material: Material,
    pub supplier_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateMaterialForm {
    pub name: String,
    pub category: String,
    pub specification: String,
    pub unit: String,
    pub stock_quantity: f64,
    pub min_stock: f64,
    pub price: f64,
    pub supplier_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct EditMaterialForm {
    pub name: String,
    pub category: String,
    pub specification: String,
    pub unit: String,
    pub stock_quantity: f64,
    pub min_stock: f64,
    pub price: f64,
    pub supplier_id: Option<i64>,
}
