use askama::Template;
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum_extra::extract::multipart::MultipartError;
use http::StatusCode;

#[derive(Template)]
#[template(path = "components/error.html")]
pub enum AppError {
    /// 301
    Redirect { url: &'static str, hx: bool },

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
            AppError::Redirect { hx, .. } => (if *hx {
                StatusCode::OK
            } else {
                StatusCode::TEMPORARY_REDIRECT
            })
            .as_u16(),
            AppError::BadRequest => StatusCode::BAD_REQUEST.as_u16(),
            AppError::Unauthorized => StatusCode::UNAUTHORIZED.as_u16(),
            AppError::Forbidden(_) => StatusCode::FORBIDDEN.as_u16(),
            AppError::NotFound => StatusCode::NOT_FOUND.as_u16(),

            AppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
        }
    }

    pub fn message(&self) -> &str {
        match self {
            AppError::Redirect { .. } => "",
            AppError::BadRequest => "Bad Request",
            AppError::Unauthorized => "Unauthorized",
            AppError::Forbidden(msg) => msg,
            AppError::NotFound => "Not Found",

            AppError::InternalServerError => "Internal Server Error",
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::Redirect { url, hx } => {
                if hx {
                    (StatusCode::OK).into_response()
                } else {
                    Redirect::to(url).into_response()
                }
            }
            err => Html(
                err.render()
                    .unwrap_or("Error rendering error page".to_string()),
            )
            .into_response(),
        }
    }
}

impl From<anyhow::Error> for AppError {
    fn from(e: anyhow::Error) -> Self {
        eprintln!("internal error: {e:#}");
        AppError::InternalServerError
    }
}

impl From<MultipartError> for AppError {
    fn from(e: MultipartError) -> Self {
        eprintln!("multipart error: {e}");
        AppError::BadRequest
    }
}