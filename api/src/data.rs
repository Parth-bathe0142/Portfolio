use anyhow::Result;
use axum::{extract::Path, response::IntoResponse};
use http::{header, StatusCode};
use shared::{
    db::{certificates::get_image, get_connection},
    errors::AppError,
};

pub async fn get_certificate_image(Path(id): Path<i64>) -> Result<impl IntoResponse, AppError> {
    let conn = get_connection()?;

    let Some(image) = get_image(&conn, id)? else {
        println!("not found {id}");
        return Err(AppError::NotFound);
    };

    Ok((
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, "image/png"),
            (header::CACHE_CONTROL, "public, max-age=86400"),
        ],
        image.data,
    ))
}
