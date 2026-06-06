use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::extract::{State, Path, Form};
use std::sync::Arc;
use tera::Context;
use tower_sessions::Session;

use crate::config::app::AppState;
use crate::models::scrap::CreateScrapForm;
use crate::services::{scrap_service, material_service};

pub async fn list(session: Session, State(state): State<Arc<AppState>>) -> Response {
    let conn = state.db.lock().unwrap();
    let scraps = scrap_service::get_all_scraps(&conn);
    drop(conn);

    let mut context = Context::new();
    context.insert("scraps", &scraps);

    let user_id: Option<i64> = session.get("user_id").await.ok().flatten();
    let username: Option<String> = session.get("username").await.ok().flatten();
    let role: Option<String> = session.get("role").await.ok().flatten();
    context.insert("logged_in", &user_id.is_some());
    context.insert("username", &username);
    context.insert("role", &role);

    let html = state.tera.render("scraps/list.html", &context).unwrap();
    Html(html).into_response()
}

pub async fn create_page(session: Session, State(state): State<Arc<AppState>>) -> Response {
    let conn = state.db.lock().unwrap();
    let materials = material_service::get_all_materials(&conn);
    drop(conn);

    let mut context = Context::new();
    context.insert("materials", &materials);

    let user_id: Option<i64> = session.get("user_id").await.ok().flatten();
    let username: Option<String> = session.get("username").await.ok().flatten();
    let role: Option<String> = session.get("role").await.ok().flatten();
    context.insert("logged_in", &user_id.is_some());
    context.insert("username", &username);
    context.insert("role", &role);

    let html = state.tera.render("scraps/create.html", &context).unwrap();
    Html(html).into_response()
}

pub async fn create(
    session: Session,
    State(state): State<Arc<AppState>>,
    Form(form): Form<CreateScrapForm>,
) -> Response {
    let conn = state.db.lock().unwrap();
    match scrap_service::create_scrap(&conn, &form) {
        Ok(_) => {
            drop(conn);
            Redirect::to("/scraps").into_response()
        }
        Err(e) => {
            let materials = material_service::get_all_materials(&conn);
            drop(conn);
            let mut context = Context::new();
            context.insert("materials", &materials);
            context.insert("error", &e);

            let user_id: Option<i64> = session.get("user_id").await.ok().flatten();
            let username: Option<String> = session.get("username").await.ok().flatten();
            let role: Option<String> = session.get("role").await.ok().flatten();
            context.insert("logged_in", &user_id.is_some());
            context.insert("username", &username);
            context.insert("role", &role);

            let html = state.tera.render("scraps/create.html", &context).unwrap();
            Html(html).into_response()
        }
    }
}

pub async fn detail(
    session: Session,
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Response {
    let conn = state.db.lock().unwrap();
    let scrap = scrap_service::get_scrap_by_id(&conn, id);
    drop(conn);

    match scrap {
        Some(s) => {
            let mut context = Context::new();
            context.insert("scrap", &s);

            let user_id: Option<i64> = session.get("user_id").await.ok().flatten();
            let username: Option<String> = session.get("username").await.ok().flatten();
            let role: Option<String> = session.get("role").await.ok().flatten();
            context.insert("logged_in", &user_id.is_some());
            context.insert("username", &username);
            context.insert("role", &role);

            let html = state.tera.render("scraps/detail.html", &context).unwrap();
            Html(html).into_response()
        }
        None => Redirect::to("/scraps").into_response(),
    }
}
