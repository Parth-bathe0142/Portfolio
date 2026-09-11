use anyhow::Result;
use askama::Template;
use shared::auth::authenticate;
use shared::db::user::Role;
use shared::utils::{redirect, templ};
use spin_sdk::http::{IntoResponse, Request, Router};
use spin_sdk::http_component;

use crate::auth::{handle_login, handle_logout, handle_register};
use crate::routes::{
    add_new_certificate, auth_page, delete_certificate, get_certificates_page_or_container,
    get_certificates_page_or_table, get_home_page_or_fragment,
};

mod auth;
mod routes;
mod templates;

#[http_component]
fn handle_admin(req: Request) -> Result<impl IntoResponse> {
    let mut router = Router::new();

    router.get("/admin", get_home_page_or_fragment);
    router.get("/admin/auth", auth_page);
    router.get("/admin/certificates", get_certificates_page_or_container);
    router.get("/admin/certificates/:page", get_certificates_page_or_table);

    router.post("/admin/register", handle_register);
    router.post("/admin/login", handle_login);
    router.post("/admin/logout", handle_logout);

    router.post("/admin/certificates", add_new_certificate);

    router.delete("/admin/certificates/:id", delete_certificate);

    Ok(router.handle(req))
}

fn templ_if_logged_in(req: Request, template: impl Template) -> Result<impl IntoResponse> {
    let htmx = req.header("Hx-Request").is_some();

    if let Err(_) = authenticate(&req, Role::Admin) {
        return Ok(redirect("/admin/auth", htmx));
    };

    templ(template)
}
