use anyhow::Result;
use axum::{
    extract::Path,
    response::{IntoResponse, Response},
};
use axum_extra::extract::Multipart;
use axum_htmx::HxRequest;
use http::StatusCode;
use serde::Deserialize;
use shared::{
    auth::extractor::AdminOnly,
    db::{
        certificates::{
            add_certificate, count_certificates, delete_certificate as delete_certificate_db,
            get_categories, get_certificates_paginated, Image,
        },
        get_connection,
        user::get_admin,
        CertificateMeta,
    },
    errors::AppError,
};

use crate::templates::{
    fragments::{CertContainer, CertTable, Home},
    AuthPage, CertPage, HomePage,
};

pub async fn auth_page() -> Result<AuthPage, AppError> {
    let conn = get_connection()?;
    let admin = get_admin(&conn)?;

    Ok(AuthPage {
        registered: admin.is_some(),
    })
}

pub async fn home(_: AdminOnly, HxRequest(hx): HxRequest) -> Result<Response, AppError> {
    if hx {
        Ok(Home.into_response())
    } else {
        Ok(HomePage.into_response())
    }
}

pub async fn get_certificates_page(
    _: AdminOnly,
    HxRequest(hx): HxRequest,
) -> Result<Response, AppError> {
    let conn = get_connection()?;
    let certificates = get_certificates_paginated(&conn, 1, 10)?;
    let categories = get_categories(&conn)?;

    let total_pages = count_certificates(&conn)? as f32 / 10.;
    let total_pages = total_pages.ceil() as u32;

    if hx {
        Ok(CertContainer {
            categories,
            certificates,
            page: 1,
            total_pages,
            error: None,
        }
        .into_response())
    } else {
        Ok(CertPage {
            categories,
            certificates,
            page: 1,
            total_pages,
            error: None,
        }
        .into_response())
    }
}

#[derive(Deserialize)]
pub(crate) struct PageQuery {
    page: Option<i64>,
}

pub async fn get_certificates_table(
    _: AdminOnly,
    Path(query): Path<PageQuery>,
    HxRequest(hx): HxRequest,
) -> Result<Response, AppError> {
    let page = query.page.unwrap_or(1);

    let conn = get_connection()?;
    let certificates = get_certificates_paginated(&conn, page, 10)?;
    let categories = get_categories(&conn)?;

    let page = page as u32;
    let total_pages = count_certificates(&conn)? as f32 / 10.;
    let total_pages = total_pages.ceil() as u32;

    if hx {
        Ok(CertTable {
            certificates,
            page,
            total_pages,
            error: None,
        }
        .into_response())
    } else {
        Ok(CertPage {
            certificates,
            categories,
            page,
            total_pages,
            error: None,
        }
        .into_response())
    }
}

pub async fn new_certificate(
    _admin: AdminOnly,
    mut multipart: Multipart,
) -> Result<CertTable, AppError> {
    let (mut name, mut issuer, mut date, mut category, mut image) = (None, None, None, None, None);

    while let Some(field) = multipart.next_field().await? {
        let field_name = field.name().map(str::to_owned);
        match field_name.as_deref() {
            Some("name") => name = Some(field.text().await?),
            Some("issuer") => issuer = Some(field.text().await?),
            Some("date") => date = Some(field.text().await?),
            Some("category") => category = Some(field.text().await?),
            Some("image") => {
                let bytes = field.bytes().await?;

                if !bytes.is_empty() {
                    image = Some(bytes);
                }
            }
            _ => {}
        }
    }

    let conn = get_connection()?;

    let mut error = None;

    if let (Some(name), Some(issuer), Some(date), Some(category), Some(image)) =
        (name, issuer, date, category, image)
    {
        let cert = CertificateMeta {
            id: None,
            name,
            issuer,
            date,
            category,
        };
        let image = Image {
            id: None,
            data: image.to_vec(),
        };
        add_certificate(&conn, cert, image)?;
    } else {
        error = Some("missing fields in form data")
    }

    let total_pages = count_certificates(&conn)? as u32;
    let certificates = get_certificates_paginated(&conn, 1, 10)?;

    Ok(CertTable {
        certificates,
        page: 1,
        total_pages,
        error,
    })
}

pub async fn delete_certificate(_: AdminOnly, Path(id): Path<i64>) -> Result<Response, AppError> {
    let conn = get_connection()?;

    let Ok(_) = delete_certificate_db(&conn, id) else {
        return Err(AppError::InternalServerError);
    };

    Ok(StatusCode::NO_CONTENT.into_response())
}
