use askama::Template;
use shared::db::CertificateMeta;

#[derive(Template)]
#[template(path = "fragments/home.html")]
pub struct Home;

#[derive(Template)]
#[template(path = "fragments/projects.html")]
pub struct Projects;

#[derive(Template)]
#[template(path = "fragments/certificates.html")]
pub struct Certificates {
	pub error: Option<&'static str>,
	pub certificates: Vec<CertificateMeta>,
	pub page: u32,
	pub total_pages: u32,
}

#[derive(Template)]
#[template(path = "fragments/not_found.html")]
pub struct NotFound;