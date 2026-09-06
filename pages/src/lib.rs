use anyhow::Result;
use askama::Template;
use http::StatusCode;
use spin_sdk::http::{IntoResponse, Request, Response, Router};
use spin_sdk::http_component;

use crate::templates::fragments::{Certificates, Home, NotFound, Projects};
use crate::templates::{CertificatesPage, HomePage, NotFoundPage, ProjectsPage};

mod templates;

/// A simple Spin HTTP component.
#[http_component]
fn handle_pages(req: Request) -> anyhow::Result<impl IntoResponse> {
    let mut router = Router::new();

    router.get("/", |_: Request, _| templ(HomePage));
    router.get("/projects", |_: Request, _| templ(ProjectsPage));
    router.get("/certificates", |_: Request, _| templ(CertificatesPage));
    
    router.get("/fragments/home", |_: Request, _| templ(Home));
    router.get("/fragments/projects", |_: Request, _| templ(Projects));
    router.get("/fragments/certificates", |_: Request, _| templ(Certificates));
    

    router.get("/fragments/*", |_: Request, _| templ(NotFound));
    router.get("/*", |_: Request, _| templ(NotFoundPage));
    
    Ok(router.handle(req))
}

fn templ(templ: impl Template) -> Result<Response> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(templ.render()?)
        .build())
}
