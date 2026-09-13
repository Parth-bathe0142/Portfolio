use shared::{
    db::{
        certificates::{count_certificates, get_certificates_paginated},
        get_connection,
    },
    utils::templ,
};
use spin_sdk::http::{IntoResponse, Params, Request};

use crate::templates::{fragments::Certificates, CertificatesPage};

pub fn get_certificates_page_or_frag(
    req: Request,
    params: Params,
) -> anyhow::Result<impl IntoResponse> {
    let conn = get_connection()?;

    let page = params
        .get("page")
        .unwrap_or("1")
        .parse::<i64>()
        .unwrap_or(1);

    let certificates = get_certificates_paginated(&conn, page, 10)?;
    let total_pages = count_certificates(&conn)?;

    let htmx = req.header("Hx-Request").is_some();

    if htmx {
        templ(Certificates {
            error: None,
            certificates,
            page: page as u32,
            total_pages: total_pages as u32,
        })
    } else {
        templ(CertificatesPage {
            error: None,
            certificates,
            page: page as u32,
            total_pages: total_pages as u32,
        })
    }
}
