use crate::models::user::{User, LoginForm, RegisterForm};
use rusqlite::Connection;

pub fn find_by_username(conn: &Connection, username: &str) -> Option<User> {
    conn.query_row(
        "SELECT id, username, password_hash, role, created_at FROM users WHERE username = ?1",
        rusqlite::params![username],
        |row| {
            Ok(User {
                id: row.get(0)?,
                username: row.get(1)?,
                password_hash: row.get(2)?,
                role: row.get(3)?,
                created_at: row.get(4)?,
            })
        },
    )
    .ok()
}

pub fn verify_password(user: &User, password: &str) -> bool {
    bcrypt::verify(password, &user.password_hash).unwrap_or(false)
}

pub fn create_user(conn: &Connection, form: &RegisterForm) -> Result<i64, String> {
    if form.password != form.confirm_password {
        return Err("两次输入的密码不一致".to_string());
    }
    if find_by_username(conn, &form.username).is_some() {
        return Err("用户名已存在".to_string());
    }
    let hash = bcrypt::hash(&form.password, bcrypt::DEFAULT_COST).map_err(|e| e.to_string())?;
    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    conn.execute(
        "INSERT INTO users (username, password_hash, role, created_at) VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params![form.username, hash, "user", now],
    )
    .map_err(|e| e.to_string())?;
    Ok(conn.last_insert_rowid())
}

pub fn find_by_id(conn: &Connection, id: i64) -> Option<User> {
    conn.query_row(
        "SELECT id, username, password_hash, role, created_at FROM users WHERE id = ?1",
        rusqlite::params![id],
        |row| {
            Ok(User {
                id: row.get(0)?,
                username: row.get(1)?,
                password_hash: row.get(2)?,
                role: row.get(3)?,
                created_at: row.get(4)?,
            })
        },
    )
    .ok()
}
