use askama::Template;
use shared::utils::templ;
use spin_sdk::http::{IntoResponse, Request, Router};
use spin_sdk::http_component;

use crate::routes::get_certificates_page_or_frag;
use crate::templates::fragments::{Home, Projects};
use crate::templates::{HomePage, NotFoundPage, ProjectsPage};

mod routes;
mod templates;

/// A simple Spin HTTP component.
#[http_component]
fn handle_pages(req: Request) -> anyhow::Result<impl IntoResponse> {
    let mut router = Router::new();

    router.get("/", |req, _| page_or_frag(&req, HomePage, Home));
    router.get("/projects", |req, _| page_or_frag(&req, ProjectsPage, Projects));
    router.get("/certificates", get_certificates_page_or_frag);

    router.get("/*", |_: Request, _| templ(NotFoundPage));
    
    Ok(router.handle(req))
}

fn page_or_frag(req: &Request, page: impl Template, fragment: impl Template) -> anyhow::Result<impl IntoResponse> {
    let htmx = req.header("Hx-Request").is_some();

    if htmx {
        Ok(templ(fragment))
    } else {
        Ok(templ(page))
    }
}
