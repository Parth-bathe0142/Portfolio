use askama::Template;
use shared::db::CertificateMeta;



#[derive(Template)]
#[template(path = "fragments/admin/home.html")]
pub struct Home;


#[derive(Template)]
#[template(path = "fragments/admin/certificates/table.html")]
pub struct CertTable {
	pub certificates: Vec<CertificateMeta>,
	pub page: u32,
	pub total_pages: u32,
	pub error: Option<&'static str>,
}

#[derive(Template)]
#[template(path = "fragments/admin/certificates/container.html")]
pub struct CertContainer {
	pub categories: Vec<String>,
	pub certificates: Vec<CertificateMeta>,
	pub page: u32,
	pub total_pages: u32,
	pub error: Option<&'static str>,
}