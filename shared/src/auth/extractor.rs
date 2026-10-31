use axum::extract::FromRequestParts;
use axum_extra::extract::{PrivateCookieJar, cookie::Key};
use spin_sdk::variables;

use crate::{
    app_state::AppState, auth::{Claims, verify_token}, db::user::Role, errors::AppError,
};

impl FromRequestParts<AppState> for Claims {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut http::request::Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let jar = PrivateCookieJar::<Key>::from_request_parts(parts, state)
            .await
            .map_err(|_| AppError::BadRequest)?;

        let token = jar
            .get("jwt_token")
            .ok_or(AppError::Unauthorized)?
            .value()
            .to_owned();

        let secret = variables::get("jwt_secret")
            .map_err(|_| AppError::InternalServerError)?
            .as_bytes()
            .to_owned();

        verify_token(&token, &secret).map_err(|_| AppError::InternalServerError)
    }
}

pub struct AdminOnly;
impl FromRequestParts<AppState> for AdminOnly {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut http::request::Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection>
    {
        let claims = match Claims::from_request_parts(parts, state).await {
         	Err(AppError::Unauthorized) => Err(AppError::Redirect{ url: "/admin/auth", hx: false }),
          	res => res
        }?;

        if claims.role == Role::Admin {
        	Ok(AdminOnly)
        } else {
        	Err(AppError::Redirect{ url: "/admin/auth", hx: false })
        }
    }
}
