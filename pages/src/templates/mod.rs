use askama::Template;
use askama_web::WebTemplate;
use shared::db::CertificateMeta;

pub mod fragments;

#[derive(Template, WebTemplate)]
#[template(path = "pages/home.html")]
pub struct HomePage;

#[derive(Template, WebTemplate)]
#[template(path = "pages/projects.html")]
pub struct ProjectsPage;

#[derive(Template, WebTemplate)]
#[template(path = "pages/certificates.html")]
pub struct CertificatesPage {
	pub error: Option<&'static str>,
	pub certificates: Vec<CertificateMeta>,
	pub page: u32,
	pub total_pages: u32,
}

#[derive(Template, WebTemplate)]
#[template(path = "pages/not_found.html")]
pub struct NotFoundPage;
