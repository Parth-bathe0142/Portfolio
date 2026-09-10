use askama::Template;
use http::StatusCode;

#[derive(Template)]
#[template(path = "fragments/admin/login_error.html")]
pub struct LoginError {
    pub code: u16,
    pub message: String,
}

impl LoginError {
    pub fn new(code: StatusCode, message: String) -> Self {
        Self { code: code.as_u16(), message }
    }
}
impl Default for LoginError {
    fn default() -> Self {
        Self::new(StatusCode::UNAUTHORIZED, "Incorrect password".to_string())
    }
}
