use askama::Template;

pub mod fragments;

#[derive(Template)]
#[template(path = "pages/home.html")]
pub struct HomePage;

#[derive(Template)]
#[template(path = "pages/projects.html")]
pub struct ProjectsPage;

#[derive(Template)]
#[template(path = "pages/certificates.html")]
pub struct CertificatesPage;

#[derive(Template)]
#[template(path = "pages/certificates.html")]
pub struct NotFoundPage;
