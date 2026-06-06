use axum::extract::Request;
use axum::middleware::Next;
use axum::response::{IntoResponse, Redirect, Response};
use tower_sessions::Session;

pub async fn require_auth(session: Session, request: Request, next: Next) -> Response {
    let user_id: Option<i64> = session.get("user_id").await.ok().flatten();
    if user_id.is_none() {
        return Redirect::to("/login").into_response();
    }
    next.run(request).await
}
