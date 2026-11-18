use axum::{extract::Path, response::IntoResponse};
use axum_extra::response::Attachment;
use http::{header, StatusCode};
use shared::{
    db::{certificates::get_image, get_connection, resumes::get_resume},
    errors::{AppError, Result},
};

pub async fn get_certificate_image(Path(id): Path<i64>) -> Result<impl IntoResponse> {
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

pub async fn get_resume_doc() -> Result<impl IntoResponse> {
    let conn = get_connection()?;
    let resume = get_resume(&conn)?;

    Ok((
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, "application/pdf"),
            (header::CACHE_CONTROL, "public, max-age=86400"),
        ],
        resume.doc,
    ))
}

pub async fn download_resume() -> Result<impl IntoResponse> {
    let conn = get_connection()?;
    let resume = get_resume(&conn)?;

    Ok(Attachment::new(resume.doc)
        .filename("Parth_Bathe_resume.pdf")
        .content_type("application/pdf"))
}
