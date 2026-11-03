use axum::{response::IntoResponse, Router};

use axum::routing::get;
use axum_htmx::HxRequest;
use spin_sdk::http::IntoResponse as SpinIntoResponse;
use spin_sdk::http_component;
use tower::ServiceExt;

use crate::{
    routes::{certificates_paginated, home, projects},
    templates::{fragments::NotFound, NotFoundPage},
};

mod routes;
mod templates;

/// A simple Spin HTTP component.
#[http_component]
async fn handle_pages(req: http::Request<Vec<u8>>) -> anyhow::Result<impl SpinIntoResponse> {
    let app = router();

    let (parts, body) = req.into_parts();
    let req = http::Request::from_parts(parts, axum::body::Body::from(body));

    let resp = app.oneshot(req).await?;
    let (parts, body) = resp.into_parts();
    let bytes = axum::body::to_bytes(body, usize::MAX).await?;

    Ok(http::Response::from_parts(parts, bytes.to_vec()))
}

fn router() -> Router {
    Router::new()
        .route("/", get(home))
        .route("/projects", get(projects))
        .route("/certificates", get(certificates_paginated))
        .fallback(get(async |HxRequest(hx): HxRequest| {
            if hx {
                NotFound.into_response()
            } else {
                NotFoundPage.into_response()
            }
        }))
}
