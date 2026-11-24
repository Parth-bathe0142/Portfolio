use axum::routing::get;
use axum::Router;
use spin_sdk::http::IntoResponse as SpinIntoResponse;
use spin_sdk::http_component;
use tower::ServiceExt;

use crate::data::{download_resume, get_certificate_image, get_resume_doc};

pub mod data;

/// A simple Spin HTTP component.
#[http_component]
async fn handle_api(req: http::Request<Vec<u8>>) -> anyhow::Result<impl SpinIntoResponse> {
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
        .route("/api/data/certificates/{id}", get(get_certificate_image))
        .route("/api/data/resume", get(get_resume_doc))
        .route("/api/data/resume/download", get(download_resume))
}
