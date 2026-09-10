use anyhow::Result;
use askama::Template;
use shared::auth::authenticate;
use shared::db::get_connection;
use shared::db::user::{Role, get_admin};
use shared::utils::{redirect, templ};
use spin_sdk::http::{IntoResponse, Params, Request, Router};
use spin_sdk::http_component;

use crate::auth::{handle_login, handle_logout, handle_register};
use crate::templates::{AuthPage, HomePage};

mod auth;
mod templates;

#[http_component]
fn handle_admin(req: Request) -> Result<impl IntoResponse> {
	println!("auth component");
    let mut router = Router::new();

	router.get("/admin", |req, _| templ_if_logged_in(req, HomePage));
	router.get("/admin/auth", auth_page);
	
	router.post("/admin/register", handle_register);
	router.post("/admin/login", handle_login);
	router.post("/admin/logout", handle_logout);

	Ok(router.handle(req))
}

fn auth_page(_: Request, _: Params) -> Result<impl IntoResponse> {
	let conn = get_connection()?;
	let admin = get_admin(&conn)?;

	templ(AuthPage { registered: admin.is_some() })
}

fn templ_if_logged_in(req: Request, template: impl Template) -> Result<impl IntoResponse>  {
	let htmx = req.header("Hx-Request").is_some();
	
	if let Err(_) = authenticate(req, Role::Admin) {
		return Ok(redirect("/admin/auth", htmx))
	};

	templ(template)
}