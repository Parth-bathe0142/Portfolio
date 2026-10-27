use axum::{
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use axum_htmx::HxRequest;
use serde::Deserialize;
use shared::db::{
    certificates::{count_certificates, get_certificates_paginated},
    get_connection,
};

use crate::templates::{
    fragments::{Certificates, Home, Projects},
    CertificatesPage, HomePage, ProjectsPage,
};

pub(crate) async fn home(HxRequest(hx): HxRequest) -> Response {
    if hx {
        Home.into_response()
    } else {
        HomePage.into_response()
    }
}

pub(crate) async fn projects(HxRequest(hx): HxRequest) -> Response {
    if hx {
        Projects.into_response()
    } else {
        ProjectsPage.into_response()
    }
}

#[derive(Deserialize)]
pub(crate) struct PageQuery {
    page: Option<i64>,
}

pub(crate) async fn certificates_paginated(
    HxRequest(htmx): HxRequest,
    Query(params): Query<PageQuery>,
) -> Result<Response, StatusCode> {
    let conn = get_connection().map_err(|e| {
        eprintln!("Failed to connect to database: {e:#}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let page = params.page.unwrap_or(1);

    let certificates = get_certificates_paginated(&conn, page, 10).map_err(|e| {
        eprintln!("Failed to fetch certificates: {e:#}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let total_pages = count_certificates(&conn).map_err(|e| {
        eprintln!("Failed to count certificates: {e:#}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let response = if htmx {
        Certificates {
            error: None,
            certificates,
            page: page as u32,
            total_pages: total_pages as u32,
        }
        .into_response()
    } else {
        CertificatesPage {
            error: None,
            certificates,
            page: page as u32,
            total_pages: total_pages as u32,
        }
        .into_response()
    };

    Ok(response)
}
