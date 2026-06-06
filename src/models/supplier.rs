use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Supplier {
    pub id: i64,
    pub name: String,
    pub contact: String,
    pub phone: String,
    pub address: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SupplierWithStats {
    pub supplier: Supplier,
    pub material_count: i64,
    pub total_value: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReconciliationItem {
    pub material_name: String,
    pub specification: String,
    pub unit: String,
    pub price: f64,
    pub stock_quantity: f64,
    pub total_price: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReconciliationSummary {
    pub supplier: Supplier,
    pub items: Vec<ReconciliationItem>,
    pub total_amount: f64,
}
