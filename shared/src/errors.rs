use askama::Template;
use http::StatusCode;

#[derive(Template)]
#[template(path = "components/error.html")]
pub enum AppError {
    /// 400
    BadRequest,
    /// 401
    Unauthorized,
    /// 403
    Forbidden(&'static str),
    /// 404
    NotFound,

    /// 500
    InternalServerError,
}

impl AppError {
    pub fn code(&self) -> u16 {
        match self {
            AppError::BadRequest => StatusCode::BAD_REQUEST.as_u16(),
            AppError::Unauthorized => StatusCode::UNAUTHORIZED.as_u16(),
            AppError::Forbidden(_) => StatusCode::FORBIDDEN.as_u16(),
            AppError::NotFound => StatusCode::NOT_FOUND.as_u16(),

            AppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
        }
    }

    pub fn message(&self) -> &str {
        match self {
            AppError::BadRequest => "Bad Request",
            AppError::Unauthorized => "Unauthorized",
            AppError::Forbidden(msg) => msg,
            AppError::NotFound => "Not Found",

            AppError::InternalServerError => "Internal Server Error",
        }
    }
}
