use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::extract::{State, Path};
use std::sync::Arc;
use tera::Context;
use tower_sessions::Session;

use crate::config::app::AppState;
use crate::services::{inventory_service, requisition_service, scrap_service, supplier_service};

pub async fn index(session: Session, State(state): State<Arc<AppState>>) -> Response {
    let user_id: Option<i64> = session.get("user_id").await.ok().flatten();
    if user_id.is_some() {
        return Redirect::to("/dashboard").into_response();
    }
    let mut context = Context::new();
    let username: Option<String> = session.get("username").await.ok().flatten();
    let role: Option<String> = session.get("role").await.ok().flatten();
    context.insert("logged_in", &user_id.is_some());
    context.insert("username", &username);
    context.insert("role", &role);
    let html = state.tera.render("index.html", &context).unwrap();
    Html(html).into_response()
}

pub async fn dashboard(session: Session, State(state): State<Arc<AppState>>) -> Response {
    let conn = state.db.lock().unwrap();

    let material_count = inventory_service::get_total_material_count(&conn);
    let warning_count = inventory_service::get_warning_count(&conn);
    let total_value = inventory_service::get_total_stock_value(&conn);
    let supplier_count = supplier_service::get_supplier_count(&conn);
    let requisitions = requisition_service::get_all_requisitions(&conn);
    let scraps = scrap_service::get_all_scraps(&conn);
    let warnings = inventory_service::get_warning_list(&conn);

    drop(conn);

    let recent_requisitions: Vec<_> = requisitions.into_iter().take(5).collect();
    let recent_scraps: Vec<_> = scraps.into_iter().take(5).collect();
    let top_warnings: Vec<_> = warnings.into_iter().take(5).collect();

    let mut context = Context::new();
    context.insert("material_count", &material_count);
    context.insert("warning_count", &warning_count);
    context.insert("total_value", &total_value);
    context.insert("supplier_count", &supplier_count);
    context.insert("recent_requisitions", &recent_requisitions);
    context.insert("recent_scraps", &recent_scraps);
    context.insert("top_warnings", &top_warnings);

    let user_id: Option<i64> = session.get("user_id").await.ok().flatten();
    let username: Option<String> = session.get("username").await.ok().flatten();
    let role: Option<String> = session.get("role").await.ok().flatten();
    context.insert("logged_in", &user_id.is_some());
    context.insert("username", &username);
    context.insert("role", &role);

    let html = state.tera.render("dashboard/overview.html", &context).unwrap();
    Html(html).into_response()
}
