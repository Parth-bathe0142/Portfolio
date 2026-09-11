use anyhow::Result;
use http::StatusCode;
use shared::{
    auth::authenticate,
    db::{
        certificates::{
            add_certificate, count_certificates, delete_certificate as delete_certificate_db,
            get_categories, get_certificates_paginated, Image,
        },
        get_connection,
        user::{get_admin, Role},
        CertificateMeta,
    },
    errors::AppError,
    utils::{decode_base64, parse_body, redirect, templ},
};
use spin_sdk::http::{IntoResponse, Params, Request, Response};

use crate::templates::{AuthPage, CertPage, HomePage, fragments::{CertContainer, CertTable, Home}};

pub fn auth_page(_: Request, _: Params) -> Result<impl IntoResponse> {
    let conn = get_connection()?;
    let admin = get_admin(&conn)?;

    templ(AuthPage {
        registered: admin.is_some(),
    })
}

pub fn get_home_page_or_fragment(req: Request, _: Params) -> Result<Response> {
    let htmx = req.header("Hx-Request").is_some();

    let Ok(_) = authenticate(&req, Role::Admin) else {
        return Ok(redirect("/admin/auth", false));
    };

    if htmx {
        templ(Home)
    } else {
    	templ(HomePage)
    }
	
}

pub fn get_certificates_page_or_container(req: Request, _: Params) -> Result<Response> {
    let htmx = req.header("Hx-Request").is_some();
    
    let Ok(_) = authenticate(&req, Role::Admin) else {
        return Ok(redirect("/admin/auth", false));
    };

    let conn = get_connection()?;
    let certificates = get_certificates_paginated(&conn, 1, 10)?;
    let categories = get_categories(&conn)?;

    let total_pages = count_certificates(&conn)? as u32;

    if htmx {
        templ(CertContainer {
            categories,
            certificates,
            page: 1,
            total_pages,
            error: None
        })
    } else {
        templ(CertPage {
            categories,
            certificates,
            page: 1,
            total_pages,
            error: None
        })
    }
}

pub fn get_certificates_page_or_table(req: Request, params: Params) -> Result<Response> {
    let htmx = req.header("Hx-Request").is_some();

    let Ok(_) = authenticate(&req, Role::Admin) else {
        return Ok(redirect("/admin/auth", false));
    };

    let page = params
        .get("page")
        .unwrap_or("1")
        .parse::<i64>()
        .unwrap_or(1);

    let conn = get_connection()?;
    let certificates = get_certificates_paginated(&conn, page, 10)?;
    let categories = get_categories(&conn)?;

    let page = page as u32;
    let total_pages = count_certificates(&conn)? as u32;

    if htmx {
        templ(CertTable {
            certificates,
            page,
            total_pages,
            error: None,
        })
    } else {
        templ(CertPage {
            certificates,
            categories,
            page,
            total_pages,
            error: None,
        })
    }
}

pub fn add_new_certificate(req: Request, _: Params) -> Result<impl IntoResponse> {
    let Ok(_) = authenticate(&req, Role::Admin) else {
        return Ok(redirect("/admin/auth", false));
    };

    let body = parse_body(&req);

    let name = body.get("name").cloned();
    let issuer = body.get("issuer").cloned();
    let date = body.get("date").cloned();
    let category = body.get("category").cloned();
    let image = body.get("image").cloned();

    let conn = get_connection()?;
    let total_pages = count_certificates(&conn)? as u32;

    let (Some(name), Some(issuer), Some(date), Some(category), Some(image)) =
        (name, issuer, date, category, image)
    else {
        let certificates = get_certificates_paginated(&conn, 1, 10)?;
        return templ(CertTable {
            certificates,
            page: 1,
            total_pages,
            error: None,
        });
    };

    let cert = CertificateMeta {
        id: None,
        name,
        issuer,
        date,
        category,
    };

    let image = Image {
        id: None,
        data: decode_base64(&image)?,
    };

    add_certificate(&conn, cert, image)?;
    let certificates = get_certificates_paginated(&conn, 1, 10)?;

    templ(CertTable {
        certificates,
        page: 1,
        total_pages,
        error: None,
    })
}

pub fn delete_certificate(req: Request, params: Params) -> Result<impl IntoResponse> {
    let Ok(_) = authenticate(&req, Role::Admin) else {
        return Ok(redirect("/admin/auth", false));
    };

    let Some(id) = params.get("id").and_then(|s| s.parse::<i64>().ok()) else {
        return templ(AppError::BadRequest);
    };

    let conn = get_connection()?;

    let Ok(_) = delete_certificate_db(&conn, id) else {
        return templ(AppError::InternalServerError);
    };

    Ok(Response::builder().status(StatusCode::OK).build())
}
