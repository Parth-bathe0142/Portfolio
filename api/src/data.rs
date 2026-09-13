use anyhow::Result;
use http::StatusCode;
use shared::{
    db::{certificates::get_image, get_connection},
    errors::AppError,
    utils::templ,
};
use spin_sdk::http::{IntoResponse, Params, Request, Response};

pub fn get_certificate_image(_: Request, params: Params) -> Result<impl IntoResponse> {
    let Some(id) = params.get("id").and_then(|s| s.parse().ok()) else {
        return templ(AppError::BadRequest);
    };

    let conn = get_connection()?;

    let Some(image) = get_image(&conn, id)? else {
    	println!("not found {id}");
        return templ(AppError::NotFound);
    };

    Ok(
        Response::builder()
            .status(StatusCode::OK)
            .header("content-type", "image/png")
            .body(image.data)
            .build()
    )
}
