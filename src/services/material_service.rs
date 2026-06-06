use crate::models::material::{Material, MaterialWithSupplier, CreateMaterialForm, EditMaterialForm};
use rusqlite::Connection;

pub fn get_all_materials(conn: &Connection) -> Vec<MaterialWithSupplier> {
    let mut stmt = conn
        .prepare(
            "SELECT m.id, m.name, m.category, m.specification, m.unit, m.stock_quantity, m.min_stock, m.price, m.supplier_id, m.created_at, m.updated_at, s.name as supplier_name FROM materials m LEFT JOIN suppliers s ON m.supplier_id = s.id ORDER BY m.created_at DESC",
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

pub fn get_material_by_id(conn: &Connection, id: i64) -> Option<MaterialWithSupplier> {
    conn.query_row(
        "SELECT m.id, m.name, m.category, m.specification, m.unit, m.stock_quantity, m.min_stock, m.price, m.supplier_id, m.created_at, m.updated_at, s.name as supplier_name FROM materials m LEFT JOIN suppliers s ON m.supplier_id = s.id WHERE m.id = ?1",
        rusqlite::params![id],
        |row| {
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
        },
    )
    .ok()
}

pub fn create_material(conn: &Connection, form: &CreateMaterialForm) -> Result<i64, String> {
    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    conn.execute(
        "INSERT INTO materials (name, category, specification, unit, stock_quantity, min_stock, price, supplier_id, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        rusqlite::params![form.name, form.category, form.specification, form.unit, form.stock_quantity, form.min_stock, form.price, form.supplier_id, &now, &now],
    )
    .map_err(|e| e.to_string())?;
    Ok(conn.last_insert_rowid())
}

pub fn update_material(conn: &Connection, id: i64, form: &EditMaterialForm) -> Result<(), String> {
    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    conn.execute(
        "UPDATE materials SET name=?1, category=?2, specification=?3, unit=?4, stock_quantity=?5, min_stock=?6, price=?7, supplier_id=?8, updated_at=?9 WHERE id=?10",
        rusqlite::params![form.name, form.category, form.specification, form.unit, form.stock_quantity, form.min_stock, form.price, form.supplier_id, &now, id],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
