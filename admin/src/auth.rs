use anyhow::Result;
use shared::{
    auth::{create_token, get_secret, hash_password, verify_password},
    db::{
        get_connection,
        user::{add_user, get_admin, Role},
        User,
    },
    errors::AppError,
    utils::{add_cookie, parse_body, redirect, templ},
};
use spin_sdk::http::{IntoResponse, Params, Request};

pub fn handle_register(req: Request, _: Params) -> Result<impl IntoResponse> {
    let body = parse_body(&req);
    let password = body.get("password").map(|p| p.trim().to_owned());

    let Some(password) = password else {
        return templ(AppError::BadRequest);
    };

    let hash = hash_password(&password)?;

    let conn = get_connection()?;

    let admin = get_admin(&conn)?;
    if admin.is_some() {
        return templ(AppError::Forbidden("Super user already exists"));
    }

    let admin = User {
        id: None,
        name: "admin".to_string(),
        role: Role::Admin,
        password: Some(hash),
    };

    let id = add_user(admin, &conn)?;
    let token = create_token(&id.to_string(), Role::Admin, &get_secret()?)?;

    Ok(add_cookie(
        redirect("admin/home", true).into_builder(),
        "jwt_token",
        &token,
        3600,
        "/",
    )
    .build())
}
pub fn handle_login(req: Request, _: Params) -> Result<impl IntoResponse> {
    let body = parse_body(&req);
    let password = body.get("password");

    let Some(password) = password else {
        return templ(AppError::BadRequest);
    };

    let conn = get_connection()?;
    let admin = get_admin(&conn)?;

    if let Some(admin) = admin {
        if let Some(hash) = admin.password {
            if verify_password(password, &hash)? {
                let salt = get_secret()?;
                let token = create_token(&admin.id.unwrap().to_string(), Role::Admin, &salt)?;

                let response = redirect("/admin", true).into_builder();
                Ok(add_cookie(response, "jwt_token", &token, 3600, "/").build())
            } else {
                templ(AppError::Unauthorized)
            }
        } else {
            templ(AppError::Unauthorized)
        }
    } else {
        templ(AppError::Forbidden("Admin not registered yet"))
    }
}

pub fn handle_logout(req: Request, _: Params) -> Result<impl IntoResponse> {
    let htmx = req.header("Hx-Request").is_some();
    Ok(add_cookie(
        redirect("admin/auth", htmx).into_builder(),
        "jwt_token",
        "0",
        0,
        "/",
    )
    .build())
}
