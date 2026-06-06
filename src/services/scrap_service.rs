use crate::models::scrap::{Scrap, ScrapWithMaterial, CreateScrapForm};
use rusqlite::Connection;

pub fn get_all_scraps(conn: &Connection) -> Vec<ScrapWithMaterial> {
    let mut stmt = conn
        .prepare(
            "SELECT s.id, s.material_id, s.quantity, s.reason, s.handler, s.status, s.created_at, m.name as material_name FROM scraps s LEFT JOIN materials m ON s.material_id = m.id ORDER BY s.created_at DESC",
        )
        .unwrap();

    stmt.query_map([], |row| {
        Ok(ScrapWithMaterial {
            scrap: Scrap {
                id: row.get(0)?,
                material_id: row.get(1)?,
                quantity: row.get(2)?,
                reason: row.get(3)?,
                handler: row.get(4)?,
                status: row.get(5)?,
                created_at: row.get(6)?,
            },
            material_name: row.get(7)?,
        })
    })
    .unwrap()
    .filter_map(|r| r.ok())
    .collect()
}

pub fn get_scrap_by_id(conn: &Connection, id: i64) -> Option<ScrapWithMaterial> {
    conn.query_row(
        "SELECT s.id, s.material_id, s.quantity, s.reason, s.handler, s.status, s.created_at, m.name as material_name FROM scraps s LEFT JOIN materials m ON s.material_id = m.id WHERE s.id = ?1",
        rusqlite::params![id],
        |row| {
            Ok(ScrapWithMaterial {
                scrap: Scrap {
                    id: row.get(0)?,
                    material_id: row.get(1)?,
                    quantity: row.get(2)?,
                    reason: row.get(3)?,
                    handler: row.get(4)?,
                    status: row.get(5)?,
                    created_at: row.get(6)?,
                },
                material_name: row.get(7)?,
            })
        },
    )
    .ok()
}

pub fn create_scrap(conn: &Connection, form: &CreateScrapForm) -> Result<i64, String> {
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
            "库存不足，当前库存: {}，处置数量: {}",
            current_stock, form.quantity
        ));
    }

    conn.execute(
        "UPDATE materials SET stock_quantity = stock_quantity - ?1, updated_at = ?2 WHERE id = ?3",
        rusqlite::params![form.quantity, &now, form.material_id],
    )
    .map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT INTO scraps (material_id, quantity, reason, handler, status, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        rusqlite::params![form.material_id, form.quantity, form.reason, form.handler, "approved", &now],
    )
    .map_err(|e| e.to_string())?;

    Ok(conn.last_insert_rowid())
}

pub fn update_scrap_status(conn: &Connection, id: i64, status: &str) -> Result<(), String> {
    conn.execute(
        "UPDATE scraps SET status = ?1 WHERE id = ?2",
        rusqlite::params![status, id],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
