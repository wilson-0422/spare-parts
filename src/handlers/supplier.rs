use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::extract::{State, Path};
use std::sync::Arc;
use tera::Context;
use tower_sessions::Session;

use crate::config::app::AppState;
use crate::services::supplier_service;

pub async fn list(session: Session, State(state): State<Arc<AppState>>) -> Response {
    let conn = state.db.lock().unwrap();
    let suppliers = supplier_service::get_all_suppliers(&conn);
    drop(conn);

    let mut context = Context::new();
    context.insert("suppliers", &suppliers);

    let user_id: Option<i64> = session.get("user_id").await.ok().flatten();
    let username: Option<String> = session.get("username").await.ok().flatten();
    let role: Option<String> = session.get("role").await.ok().flatten();
    context.insert("logged_in", &user_id.is_some());
    context.insert("username", &username);
    context.insert("role", &role);

    let html = state.tera.render("suppliers/list.html", &context).unwrap();
    Html(html).into_response()
}

pub async fn detail(
    session: Session,
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Response {
    let conn = state.db.lock().unwrap();
    let supplier = supplier_service::get_supplier_by_id(&conn, id);
    drop(conn);

    match supplier {
        Some(s) => {
            let mut context = Context::new();
            context.insert("supplier", &s);

            let user_id: Option<i64> = session.get("user_id").await.ok().flatten();
            let username: Option<String> = session.get("username").await.ok().flatten();
            let role: Option<String> = session.get("role").await.ok().flatten();
            context.insert("logged_in", &user_id.is_some());
            context.insert("username", &username);
            context.insert("role", &role);

            let html = state.tera.render("suppliers/detail.html", &context).unwrap();
            Html(html).into_response()
        }
        None => Redirect::to("/suppliers").into_response(),
    }
}

pub async fn reconcile_page(
    session: Session,
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Response {
    let conn = state.db.lock().unwrap();
    let reconciliation = supplier_service::get_reconciliation(&conn, id);
    drop(conn);

    match reconciliation {
        Some(r) => {
            let mut context = Context::new();
            context.insert("reconciliation", &r);

            let user_id: Option<i64> = session.get("user_id").await.ok().flatten();
            let username: Option<String> = session.get("username").await.ok().flatten();
            let role: Option<String> = session.get("role").await.ok().flatten();
            context.insert("logged_in", &user_id.is_some());
            context.insert("username", &username);
            context.insert("role", &role);

            let html = state.tera.render("suppliers/reconcile.html", &context).unwrap();
            Html(html).into_response()
        }
        None => Redirect::to("/suppliers").into_response(),
    }
}

pub async fn reconcile(
    session: Session,
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Response {
    reconcile_page(session, state, Path(id)).await
}
