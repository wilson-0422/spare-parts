use rusqlite::Connection;

pub fn init_database() -> Result<Connection, rusqlite::Error> {
    let conn = Connection::open("spare_parts.db")?;
    conn.execute_batch(
        "PRAGMA journal_mode=WAL;
         PRAGMA foreign_keys=ON;

         CREATE TABLE IF NOT EXISTS users (
             id INTEGER PRIMARY KEY AUTOINCREMENT,
             username TEXT NOT NULL UNIQUE,
             password_hash TEXT NOT NULL,
             role TEXT NOT NULL DEFAULT 'user',
             created_at TEXT NOT NULL
         );

         CREATE TABLE IF NOT EXISTS suppliers (
             id INTEGER PRIMARY KEY AUTOINCREMENT,
             name TEXT NOT NULL,
             contact TEXT NOT NULL,
             phone TEXT NOT NULL,
             address TEXT,
             created_at TEXT NOT NULL
         );

         CREATE TABLE IF NOT EXISTS materials (
             id INTEGER PRIMARY KEY AUTOINCREMENT,
             name TEXT NOT NULL,
             category TEXT NOT NULL,
             specification TEXT,
             unit TEXT NOT NULL,
             stock_quantity REAL NOT NULL DEFAULT 0,
             min_stock REAL NOT NULL DEFAULT 0,
             price REAL NOT NULL DEFAULT 0,
             supplier_id INTEGER REFERENCES suppliers(id),
             created_at TEXT NOT NULL,
             updated_at TEXT NOT NULL
         );

         CREATE TABLE IF NOT EXISTS requisitions (
             id INTEGER PRIMARY KEY AUTOINCREMENT,
             material_id INTEGER NOT NULL REFERENCES materials(id),
             quantity REAL NOT NULL,
             department TEXT NOT NULL,
             applicant TEXT NOT NULL,
             purpose TEXT,
             status TEXT NOT NULL DEFAULT 'pending',
             created_at TEXT NOT NULL
         );

         CREATE TABLE IF NOT EXISTS scraps (
             id INTEGER PRIMARY KEY AUTOINCREMENT,
             material_id INTEGER NOT NULL REFERENCES materials(id),
             quantity REAL NOT NULL,
             reason TEXT NOT NULL,
             handler TEXT NOT NULL,
             status TEXT NOT NULL DEFAULT 'pending',
             created_at TEXT NOT NULL
         );",
    )?;
    Ok(conn)
}
