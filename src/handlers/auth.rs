use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::extract::{State, Form};
use std::sync::Arc;
use tera::Context;
use tower_sessions::Session;

use crate::config::app::AppState;
use crate::models::user::{LoginForm, RegisterForm};
use crate::services::user_service;

async fn insert_auth_context(session: &Session, context: &mut Context) {
    let user_id: Option<i64> = session.get("user_id").await.ok().flatten();
    let username: Option<String> = session.get("username").await.ok().flatten();
    let role: Option<String> = session.get("role").await.ok().flatten();
    context.insert("logged_in", &user_id.is_some());
    context.insert("username", &username);
    context.insert("role", &role);
}

pub async fn login_page(session: Session, State(state): State<Arc<AppState>>) -> Response {
    let user_id: Option<i64> = session.get("user_id").await.ok().flatten();
    if user_id.is_some() {
        return Redirect::to("/dashboard").into_response();
    }
    let mut context = Context::new();
    insert_auth_context(&session, &mut context).await;
    let html = state.tera.render("auth/login.html", &context).unwrap();
    Html(html).into_response()
}

pub async fn login(
    session: Session,
    State(state): State<Arc<AppState>>,
    Form(form): Form<LoginForm>,
) -> Response {
    let conn = state.db.lock().unwrap();
    match user_service::find_by_username(&conn, &form.username) {
        Some(user) if user_service::verify_password(&user, &form.password) => {
            drop(conn);
            session.insert("user_id", user.id).await.unwrap();
            session.insert("username", &user.username).await.unwrap();
            session.insert("role", &user.role).await.unwrap();
            Redirect::to("/dashboard").into_response()
        }
        _ => {
            drop(conn);
            let mut context = Context::new();
            insert_auth_context(&session, &mut context).await;
            context.insert("error", "用户名或密码错误");
            let html = state.tera.render("auth/login.html", &context).unwrap();
            Html(html).into_response()
        }
    }
}

pub async fn register_page(session: Session, State(state): State<Arc<AppState>>) -> Response {
    let mut context = Context::new();
    insert_auth_context(&session, &mut context).await;
    let html = state.tera.render("auth/register.html", &context).unwrap();
    Html(html).into_response()
}

pub async fn register(
    session: Session,
    State(state): State<Arc<AppState>>,
    Form(form): Form<RegisterForm>,
) -> Response {
    let conn = state.db.lock().unwrap();
    match user_service::create_user(&conn, &form) {
        Ok(_) => {
            drop(conn);
            let mut context = Context::new();
            insert_auth_context(&session, &mut context).await;
            context.insert("success", "注册成功，请登录");
            let html = state.tera.render("auth/login.html", &context).unwrap();
            Html(html).into_response()
        }
        Err(e) => {
            drop(conn);
            let mut context = Context::new();
            insert_auth_context(&session, &mut context).await;
            context.insert("error", &e);
            let html = state.tera.render("auth/register.html", &context).unwrap();
            Html(html).into_response()
        }
    }
}

pub async fn logout(session: Session) -> Response {
    session.remove::<i64>("user_id").await.ok();
    session.remove::<String>("username").await.ok();
    session.remove::<String>("role").await.ok();
    Redirect::to("/login").into_response()
}
