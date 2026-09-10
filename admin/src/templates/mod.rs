use askama::Template;

pub mod fragments;

#[derive(Template)]
#[template(path = "pages/admin/auth.html")]
pub struct AuthPage {
	pub registered: bool
}

#[derive(Template)]
#[template(path = "pages/admin/home.html")]
pub struct HomePage;