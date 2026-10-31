use anyhow::Result;
use axum::response::{IntoResponse, Redirect, Response};
use axum_extra::extract::{
    cookie::{Cookie, Key, SameSite},
    Form, PrivateCookieJar,
};
use axum_htmx::{HxRedirect, HxRequest};
use serde::Deserialize;
use shared::{
    auth::{create_token, get_secret, hash_password, verify_password},
    db::{
        get_connection,
        user::{add_user, get_admin, Role},
        User,
    },
    errors::AppError,
};

#[derive(Deserialize)]
pub struct AuthPayload {
    password: String,
}

pub async fn handle_register(
    jar: PrivateCookieJar<Key>,
    Form(payload): Form<AuthPayload>,
) -> Result<impl IntoResponse, AppError> {
    let password = payload.password;
    if password.is_empty() {
        return Err(AppError::BadRequest);
    }

    let hash = hash_password(&password)?;

    let conn = get_connection()?;

    let admin = get_admin(&conn)?;
    if admin.is_some() {
        return Err(AppError::Forbidden("Super user already exists"));
    }

    let admin = User {
        id: None,
        name: "admin".to_string(),
        role: Role::Admin,
        password: Some(hash),
    };

    let id = add_user(admin, &conn)?;
    let token = create_token(&id.to_string(), Role::Admin, &get_secret()?)?;

    let jar = jar.add(Cookie::new("jwt_token", token));

    Ok((jar, Redirect::to("/admin")))
}
pub async fn handle_login(
    HxRequest(hx): HxRequest,
    jar: PrivateCookieJar<Key>,
    Form(payload): Form<AuthPayload>,
) -> Result<impl IntoResponse, AppError> {
    let password = payload.password;
    if password.is_empty() {
        return Err(AppError::BadRequest);
    }

    let conn = get_connection()?;
    let admin = get_admin(&conn)?;

    let Some(admin) = admin else {
        return Err(AppError::Forbidden("Admin not registered yet"));
    };

    let Some(hash) = admin.password else {
        return Err(AppError::InternalServerError);
    };

    if !verify_password(&password, &hash)? {
        return Err(AppError::Unauthorized);
    }

    let salt = get_secret()?;
    let token = create_token(&admin.id.unwrap().to_string(), Role::Admin, &salt)?;

    let cookie = Cookie::build(("jwt_token", token))
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .path("/")
        .build();

    let jar = jar.add(cookie);

    Ok(if hx {
        (HxRedirect("/admin".to_string()), jar).into_response()
    } else {
        (jar, Redirect::to("/admin")).into_response()
    })
}

pub async fn handle_logout(jar: PrivateCookieJar, HxRequest(hx): HxRequest) -> Response {
    let jar = jar.remove(Cookie::build("jwt_token").path("/"));

    if hx {
        (HxRedirect("/admin/auth".to_string()), jar).into_response()
    } else {
        (jar, Redirect::to("/admin/auth")).into_response()
    }
}
