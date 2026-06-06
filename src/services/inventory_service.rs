use crate::models::material::{Material, MaterialWithSupplier};
use rusqlite::Connection;

pub fn get_inventory_list(conn: &Connection) -> Vec<MaterialWithSupplier> {
    let mut stmt = conn
        .prepare(
            "SELECT m.id, m.name, m.category, m.specification, m.unit, m.stock_quantity, m.min_stock, m.price, m.supplier_id, m.created_at, m.updated_at, s.name as supplier_name FROM materials m LEFT JOIN suppliers s ON m.supplier_id = s.id ORDER BY m.category, m.name",
        )
        .unwrap();

    stmt.query_map([], |row| {
        Ok(MaterialWithSupplier {
            material: Material {
                id: row.get(0)?,
                name: row.get(1)?,
                category: row.get(2)?,
                specification: row.get(3)?,
                unit: row.get(4)?,
                stock_quantity: row.get(5)?,
                min_stock: row.get(6)?,
                price: row.get(7)?,
                supplier_id: row.get(8)?,
                created_at: row.get(9)?,
                updated_at: row.get(10)?,
            },
            supplier_name: row.get(11)?,
        })
    })
    .unwrap()
    .filter_map(|r| r.ok())
    .collect()
}

pub fn get_warning_list(conn: &Connection) -> Vec<MaterialWithSupplier> {
    let mut stmt = conn
        .prepare(
            "SELECT m.id, m.name, m.category, m.specification, m.unit, m.stock_quantity, m.min_stock, m.price, m.supplier_id, m.created_at, m.updated_at, s.name as supplier_name FROM materials m LEFT JOIN suppliers s ON m.supplier_id = s.id WHERE m.stock_quantity <= m.min_stock ORDER BY (m.min_stock - m.stock_quantity) DESC",
        )
        .unwrap();

    stmt.query_map([], |row| {
        Ok(MaterialWithSupplier {
            material: Material {
                id: row.get(0)?,
                name: row.get(1)?,
                category: row.get(2)?,
                specification: row.get(3)?,
                unit: row.get(4)?,
                stock_quantity: row.get(5)?,
                min_stock: row.get(6)?,
                price: row.get(7)?,
                supplier_id: row.get(8)?,
                created_at: row.get(9)?,
                updated_at: row.get(10)?,
            },
            supplier_name: row.get(11)?,
        })
    })
    .unwrap()
    .filter_map(|r| r.ok())
    .collect()
}

pub fn get_total_material_count(conn: &Connection) -> i64 {
    conn.query_row("SELECT COUNT(*) FROM materials", [], |row| row.get(0))
        .unwrap_or(0)
}

pub fn get_warning_count(conn: &Connection) -> i64 {
    conn.query_row(
        "SELECT COUNT(*) FROM materials WHERE stock_quantity <= min_stock",
        [],
        |row| row.get(0),
    )
    .unwrap_or(0)
}

pub fn get_total_stock_value(conn: &Connection) -> f64 {
    conn.query_row(
        "SELECT COALESCE(SUM(stock_quantity * price), 0) FROM materials",
        [],
        |row| row.get(0),
    )
    .unwrap_or(0.0)
}
