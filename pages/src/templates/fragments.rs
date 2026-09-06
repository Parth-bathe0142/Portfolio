use askama::Template;

#[derive(Template)]
#[template(path = "fragments/home.html")]
pub struct Home;

#[derive(Template)]
#[template(path = "fragments/projects.html")]
pub struct Projects;

#[derive(Template)]
#[template(path = "fragments/certificates.html")]
pub struct Certificates;

#[derive(Template)]
#[template(path = "fragments/not_found.html")]
pub struct NotFound;