mod config;
mod handlers;
mod middleware;
mod models;
mod services;

use std::sync::Arc;

use axum::Router;
use axum::routing::{get, post};
use axum::middleware;
use tower_http::services::ServeDir;
use tower_sessions::{MemoryStore, SessionManagerLayer};

use config::app::AppState;
use config::database::init_database;
use config::seed::seed_data;

#[tokio::main]
async fn main() {
    let conn = init_database().expect("数据库初始化失败");
    seed_data(&conn).expect("种子数据插入失败");

    let tera = tera::Tera::new("templates/**/*").expect("模板初始化失败");

    let state = Arc::new(AppState {
        db: std::sync::Mutex::new(conn),
        tera,
    });

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store);

    let public_routes = Router::new()
        .route("/", get(handlers::home::index))
        .route("/login", get(handlers::auth::login_page).post(handlers::auth::login))
        .route("/register", get(handlers::auth::register_page).post(handlers::auth::register));

    let protected_routes = Router::new()
        .route("/logout", get(handlers::auth::logout))
        .route("/dashboard", get(handlers::home::dashboard))
        .route("/materials", get(handlers::material::list))
        .route("/materials/create", get(handlers::material::create_page).post(handlers::material::create))
        .route("/materials/:id", get(handlers::material::detail))
        .route("/materials/:id/edit", get(handlers::material::edit_page).post(handlers::material::edit))
        .route("/requisitions", get(handlers::requisition::list))
        .route("/requisitions/create", get(handlers::requisition::create_page).post(handlers::requisition::create))
        .route("/requisitions/:id", get(handlers::requisition::detail))
        .route("/scraps", get(handlers::scrap::list))
        .route("/scraps/create", get(handlers::scrap::create_page).post(handlers::scrap::create))
        .route("/scraps/:id", get(handlers::scrap::detail))
        .route("/inventory", get(handlers::inventory::list))
        .route("/inventory/warning", get(handlers::inventory::warning))
        .route("/suppliers", get(handlers::supplier::list))
        .route("/suppliers/:id", get(handlers::supplier::detail))
        .route("/suppliers/:id/reconcile", get(handlers::supplier::reconcile_page).post(handlers::supplier::reconcile))
        .layer(middleware::from_fn(middleware::auth::require_auth));

    let app = Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .nest_service("/static", ServeDir::new("static"))
        .layer(session_layer)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("服务器启动在 http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
