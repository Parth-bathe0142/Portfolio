use anyhow::Result;
use axum::routing::{get, post};
use axum::Router;
use shared::app_state::AppState;
use spin_sdk::http::IntoResponse as SpinIntoResponse;
use spin_sdk::http_component;
use tower::ServiceExt;

use crate::auth::{handle_login, handle_logout, handle_register};
use crate::routes::{
    auth_page, delete_certificate, get_certificates_page, get_certificates_table, home,
    new_certificate,
};

mod auth;
mod routes;
mod templates;

#[http_component]
async fn handle_admin(req: http::Request<Vec<u8>>) -> Result<impl SpinIntoResponse> {
    let app = router(AppState::new().expect("secret variable jwt_secret not set"));

    let (parts, body) = req.into_parts();
    let req = http::Request::from_parts(parts, axum::body::Body::from(body));

    let resp = app.oneshot(req).await?;
    let (parts, body) = resp.into_parts();
    let bytes = axum::body::to_bytes(body, usize::MAX).await?;

    Ok(http::Response::from_parts(parts, bytes.to_vec()))
}

fn router(state: AppState) -> Router {
    Router::new()
        .route("/admin", get(home))
        .route("/admin/auth", get(auth_page))
        .route(
            "/admin/certificates",
            get(get_certificates_page).post(new_certificate),
        )
        .route(
            "/admin/certificates/{page}",
            get(get_certificates_table).delete(delete_certificate),
        )
        .route("/admin/register", post(handle_register))
        .route("/admin/login", post(handle_login))
        .route("/admin/logout", post(handle_logout))
        .with_state(state)
}
