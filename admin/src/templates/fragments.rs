use askama::Template;
use askama_web::WebTemplate;
use shared::db::CertificateMeta;

#[derive(Template, WebTemplate)]
#[template(path = "fragments/admin/home.html")]
pub struct Home;

#[derive(Template, WebTemplate)]
#[template(path = "fragments/admin/certificates/table.html")]
pub struct CertTable {
    pub certificates: Vec<CertificateMeta>,
    pub page: u32,
    pub total_pages: u32,
    pub error: Option<&'static str>,
}

#[derive(Template, WebTemplate)]
#[template(path = "fragments/admin/certificates/container.html")]
pub struct CertContainer {
    pub categories: Vec<String>,
    pub certificates: Vec<CertificateMeta>,
    pub page: u32,
    pub total_pages: u32,
    pub error: Option<&'static str>,
}
