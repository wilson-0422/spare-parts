use axum::response::{Html, IntoResponse, Response};
use axum::extract::State;
use std::sync::Arc;
use tera::Context;
use tower_sessions::Session;

use crate::config::app::AppState;
use crate::services::inventory_service;

pub async fn list(session: Session, State(state): State<Arc<AppState>>) -> Response {
    let conn = state.db.lock().unwrap();
    let materials = inventory_service::get_inventory_list(&conn);
    drop(conn);

    let mut context = Context::new();
    context.insert("materials", &materials);

    let user_id: Option<i64> = session.get("user_id").await.ok().flatten();
    let username: Option<String> = session.get("username").await.ok().flatten();
    let role: Option<String> = session.get("role").await.ok().flatten();
    context.insert("logged_in", &user_id.is_some());
    context.insert("username", &username);
    context.insert("role", &role);

    let html = state.tera.render("inventory/list.html", &context).unwrap();
    Html(html).into_response()
}

pub async fn warning(session: Session, State(state): State<Arc<AppState>>) -> Response {
    let conn = state.db.lock().unwrap();
    let materials = inventory_service::get_warning_list(&conn);
    drop(conn);

    let mut context = Context::new();
    context.insert("materials", &materials);

    let user_id: Option<i64> = session.get("user_id").await.ok().flatten();
    let username: Option<String> = session.get("username").await.ok().flatten();
    let role: Option<String> = session.get("role").await.ok().flatten();
    context.insert("logged_in", &user_id.is_some());
    context.insert("username", &username);
    context.insert("role", &role);

    let html = state.tera.render("inventory/warning.html", &context).unwrap();
    Html(html).into_response()
}
