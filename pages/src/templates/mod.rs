use askama::Template;
use shared::db::CertificateMeta;

pub mod fragments;

#[derive(Template)]
#[template(path = "pages/home.html")]
pub struct HomePage;

#[derive(Template)]
#[template(path = "pages/projects.html")]
pub struct ProjectsPage;

#[derive(Template)]
#[template(path = "pages/certificates.html")]
pub struct CertificatesPage {
	pub error: Option<&'static str>,
	pub certificates: Vec<CertificateMeta>,
	pub page: u32,
	pub total_pages: u32,
}

#[derive(Template)]
#[template(path = "pages/not_found.html")]
pub struct NotFoundPage;
