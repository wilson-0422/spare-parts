use crate::models::requisition::{Requisition, RequisitionWithMaterial, CreateRequisitionForm};
use rusqlite::Connection;

pub fn get_all_requisitions(conn: &Connection) -> Vec<RequisitionWithMaterial> {
    let mut stmt = conn
        .prepare(
            "SELECT r.id, r.material_id, r.quantity, r.department, r.applicant, r.purpose, r.status, r.created_at, m.name as material_name FROM requisitions r LEFT JOIN materials m ON r.material_id = m.id ORDER BY r.created_at DESC",
        )
        .unwrap();

    stmt.query_map([], |row| {
        Ok(RequisitionWithMaterial {
            requisition: Requisition {
                id: row.get(0)?,
                material_id: row.get(1)?,
                quantity: row.get(2)?,
                department: row.get(3)?,
                applicant: row.get(4)?,
                purpose: row.get(5)?,
                status: row.get(6)?,
                created_at: row.get(7)?,
            },
            material_name: row.get(8)?,
        })
    })
    .unwrap()
    .filter_map(|r| r.ok())
    .collect()
}

pub fn get_requisition_by_id(conn: &Connection, id: i64) -> Option<RequisitionWithMaterial> {
    conn.query_row(
        "SELECT r.id, r.material_id, r.quantity, r.department, r.applicant, r.purpose, r.status, r.created_at, m.name as material_name FROM requisitions r LEFT JOIN materials m ON r.material_id = m.id WHERE r.id = ?1",
        rusqlite::params![id],
        |row| {
            Ok(RequisitionWithMaterial {
                requisition: Requisition {
                    id: row.get(0)?,
                    material_id: row.get(1)?,
                    quantity: row.get(2)?,
                    department: row.get(3)?,
                    applicant: row.get(4)?,
                    purpose: row.get(5)?,
                    status: row.get(6)?,
                    created_at: row.get(7)?,
                },
                material_name: row.get(8)?,
            })
        },
    )
    .ok()
}

pub fn create_requisition(conn: &Connection, form: &CreateRequisitionForm) -> Result<i64, String> {
    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let current_stock: f64 = conn
        .query_row(
            "SELECT stock_quantity FROM materials WHERE id = ?1",
            rusqlite::params![form.material_id],
            |row| row.get(0),
        )
        .map_err(|e| format!("原料不存在: {}", e))?;

    if current_stock < form.quantity {
        return Err(format!(
            "库存不足，当前库存: {}，申请数量: {}",
            current_stock, form.quantity
        ));
    }

    conn.execute(
        "UPDATE materials SET stock_quantity = stock_quantity - ?1, updated_at = ?2 WHERE id = ?3",
        rusqlite::params![form.quantity, &now, form.material_id],
    )
    .map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT INTO requisitions (material_id, quantity, department, applicant, purpose, status, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        rusqlite::params![form.material_id, form.quantity, form.department, form.applicant, form.purpose, "approved", &now],
    )
    .map_err(|e| e.to_string())?;

    Ok(conn.last_insert_rowid())
}

pub fn update_requisition_status(conn: &Connection, id: i64, status: &str) -> Result<(), String> {
    conn.execute(
        "UPDATE requisitions SET status = ?1 WHERE id = ?2",
        rusqlite::params![status, id],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
