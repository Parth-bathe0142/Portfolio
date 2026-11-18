use axum::{
    extract::Query,
    response::{IntoResponse, Response},
};
use axum_htmx::HxRequest;
use serde::Deserialize;
use shared::{db::{
    certificates::{count_certificates, get_certificates_paginated}, get_connection, resumes::get_resume,
}, errors::Result};

use crate::templates::{
    CertificatesPage, HomePage, ProjectsPage, ResumePage, fragments::{Certificates, Home, Projects, Resume},
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
    HxRequest(hx): HxRequest,
    Query(params): Query<PageQuery>,
) -> Result<Response> {
    let conn = get_connection()?;

    let page = params.page.unwrap_or(1);

    let certificates = get_certificates_paginated(&conn, page, 10)?;

    let total_pages = count_certificates(&conn)?;

    let response = if hx {
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

pub async fn resume(HxRequest(hx): HxRequest) -> Result<Response> {
	let conn = get_connection()?;
	let resume = get_resume(&conn)?;

	if hx {
		Ok(Resume { id: resume.id }.into_response())
	} else {
		Ok(ResumePage { id: resume.id }.into_response())
	}
}