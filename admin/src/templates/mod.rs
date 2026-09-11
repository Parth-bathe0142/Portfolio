use askama::Template;
use shared::db::CertificateMeta;

pub mod fragments;

#[derive(Template)]
#[template(path = "pages/admin/auth.html")]
pub struct AuthPage {
	pub registered: bool
}

#[derive(Template)]
#[template(path = "pages/admin/home.html")]
pub struct HomePage;

#[derive(Template)]
#[template(path = "pages/admin/certificates.html")]
pub struct CertPage {
	pub certificates: Vec<CertificateMeta>, 
	pub categories: Vec<String>,
	pub page: u32,
	pub total_pages: u32,
	pub error: Option<&'static str>,
}