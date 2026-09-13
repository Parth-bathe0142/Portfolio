use spin_sdk::http::{IntoResponse, Request, Router};
use spin_sdk::http_component;

pub mod data;

/// A simple Spin HTTP component.
#[http_component]
fn handle_api(req: Request) -> anyhow::Result<impl IntoResponse> {
	println!("{}",req.uri());
	
    let mut router = Router::new();

    router.get("api/data/certificate/:id", data::get_certificate_image);

    Ok(router.handle(req))
}
