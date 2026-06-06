use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::extract::{State, Path, Form};
use std::sync::Arc;
use tera::Context;
use tower_sessions::Session;

use crate::config::app::AppState;
use crate::models::material::{CreateMaterialForm, EditMaterialForm};
use crate::services::{material_service, supplier_service};

pub async fn list(session: Session, State(state): State<Arc<AppState>>) -> Response {
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

    let html = state.tera.render("materials/list.html", &context).unwrap();
    Html(html).into_response()
}

pub async fn create_page(session: Session, State(state): State<Arc<AppState>>) -> Response {
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

    let html = state.tera.render("materials/create.html", &context).unwrap();
    Html(html).into_response()
}

pub async fn create(
    session: Session,
    State(state): State<Arc<AppState>>,
    Form(form): Form<CreateMaterialForm>,
) -> Response {
    let conn = state.db.lock().unwrap();
    match material_service::create_material(&conn, &form) {
        Ok(_) => {
            drop(conn);
            Redirect::to("/materials").into_response()
        }
        Err(e) => {
            let suppliers = supplier_service::get_all_suppliers(&conn);
            drop(conn);
            let mut context = Context::new();
            context.insert("suppliers", &suppliers);
            context.insert("error", &e);

            let user_id: Option<i64> = session.get("user_id").await.ok().flatten();
            let username: Option<String> = session.get("username").await.ok().flatten();
            let role: Option<String> = session.get("role").await.ok().flatten();
            context.insert("logged_in", &user_id.is_some());
            context.insert("username", &username);
            context.insert("role", &role);

            let html = state.tera.render("materials/create.html", &context).unwrap();
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
    let material = material_service::get_material_by_id(&conn, id);
    drop(conn);

    match material {
        Some(m) => {
            let mut context = Context::new();
            context.insert("material", &m);

            let user_id: Option<i64> = session.get("user_id").await.ok().flatten();
            let username: Option<String> = session.get("username").await.ok().flatten();
            let role: Option<String> = session.get("role").await.ok().flatten();
            context.insert("logged_in", &user_id.is_some());
            context.insert("username", &username);
            context.insert("role", &role);

            let html = state.tera.render("materials/detail.html", &context).unwrap();
            Html(html).into_response()
        }
        None => Redirect::to("/materials").into_response(),
    }
}

pub async fn edit_page(
    session: Session,
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Response {
    let conn = state.db.lock().unwrap();
    let material = material_service::get_material_by_id(&conn, id);
    let suppliers = supplier_service::get_all_suppliers(&conn);
    drop(conn);

    match material {
        Some(m) => {
            let mut context = Context::new();
            context.insert("material", &m);
            context.insert("suppliers", &suppliers);

            let user_id: Option<i64> = session.get("user_id").await.ok().flatten();
            let username: Option<String> = session.get("username").await.ok().flatten();
            let role: Option<String> = session.get("role").await.ok().flatten();
            context.insert("logged_in", &user_id.is_some());
            context.insert("username", &username);
            context.insert("role", &role);

            let html = state.tera.render("materials/edit.html", &context).unwrap();
            Html(html).into_response()
        }
        None => Redirect::to("/materials").into_response(),
    }
}

pub async fn edit(
    session: Session,
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    Form(form): Form<EditMaterialForm>,
) -> Response {
    let conn = state.db.lock().unwrap();
    match material_service::update_material(&conn, id, &form) {
        Ok(_) => {
            drop(conn);
            Redirect::to("/materials").into_response()
        }
        Err(e) => {
            let material = material_service::get_material_by_id(&conn, id);
            let suppliers = supplier_service::get_all_suppliers(&conn);
            drop(conn);
            let mut context = Context::new();
            context.insert("material", &material);
            context.insert("suppliers", &suppliers);
            context.insert("error", &e);

            let user_id: Option<i64> = session.get("user_id").await.ok().flatten();
            let username: Option<String> = session.get("username").await.ok().flatten();
            let role: Option<String> = session.get("role").await.ok().flatten();
            context.insert("logged_in", &user_id.is_some());
            context.insert("username", &username);
            context.insert("role", &role);

            let html = state.tera.render("materials/edit.html", &context).unwrap();
            Html(html).into_response()
        }
    }
}
