use crate::models::supplier::{Supplier, SupplierWithStats, ReconciliationItem, ReconciliationSummary};
use rusqlite::Connection;

pub fn get_all_suppliers(conn: &Connection) -> Vec<SupplierWithStats> {
    let mut stmt = conn
        .prepare(
            "SELECT s.id, s.name, s.contact, s.phone, s.address, s.created_at, COUNT(m.id) as material_count, COALESCE(SUM(m.stock_quantity * m.price), 0) as total_value FROM suppliers s LEFT JOIN materials m ON s.id = m.supplier_id GROUP BY s.id ORDER BY s.created_at DESC",
        )
        .unwrap();

    stmt.query_map([], |row| {
        Ok(SupplierWithStats {
            supplier: Supplier {
                id: row.get(0)?,
                name: row.get(1)?,
                contact: row.get(2)?,
                phone: row.get(3)?,
                address: row.get(4)?,
                created_at: row.get(5)?,
            },
            material_count: row.get(6)?,
            total_value: row.get(7)?,
        })
    })
    .unwrap()
    .filter_map(|r| r.ok())
    .collect()
}

pub fn get_supplier_by_id(conn: &Connection, id: i64) -> Option<Supplier> {
    conn.query_row(
        "SELECT id, name, contact, phone, address, created_at FROM suppliers WHERE id = ?1",
        rusqlite::params![id],
        |row| {
            Ok(Supplier {
                id: row.get(0)?,
                name: row.get(1)?,
                contact: row.get(2)?,
                phone: row.get(3)?,
                address: row.get(4)?,
                created_at: row.get(5)?,
            })
        },
    )
    .ok()
}

pub fn get_reconciliation(conn: &Connection, supplier_id: i64) -> Option<ReconciliationSummary> {
    let supplier = get_supplier_by_id(conn, supplier_id)?;

    let mut stmt = conn
        .prepare(
            "SELECT name, specification, unit, price, stock_quantity, (price * stock_quantity) as total_price FROM materials WHERE supplier_id = ?1 ORDER BY name",
        )
        .unwrap();

    let items: Vec<ReconciliationItem> = stmt
        .query_map(rusqlite::params![supplier_id], |row| {
            Ok(ReconciliationItem {
                material_name: row.get(0)?,
                specification: row.get(1)?,
                unit: row.get(2)?,
                price: row.get(3)?,
                stock_quantity: row.get(4)?,
                total_price: row.get(5)?,
            })
        })
        .unwrap()
        .filter_map(|r| r.ok())
        .collect();

    let total_amount: f64 = items.iter().map(|i| i.total_price).sum();

    Some(ReconciliationSummary {
        supplier,
        items,
        total_amount,
    })
}

pub fn get_supplier_count(conn: &Connection) -> i64 {
    conn.query_row("SELECT COUNT(*) FROM suppliers", [], |row| row.get(0))
        .unwrap_or(0)
}
