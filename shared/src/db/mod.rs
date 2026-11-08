use spin_sdk::sqlite::{Connection, Row};

pub mod certificates;
pub mod resumes;
pub mod user;

pub use certificates::CertificateMeta;
pub use user::User;

use crate::errors::{AppError, Result};

pub fn get_connection() -> Result<Connection> {
    let conn = Connection::open_default().map_err(|_| AppError::InternalServerError)?;

    conn.execute(
        "
		CREATE TABLE IF NOT EXISTS p_users (
			id INTEGER PRIMARY KEY AUTOINCREMENT,
			name TEXT NOT NULL,
			role TEXT DEFAULT('viewer'),
			password TEXT NOT NULL
		)
	",
        &[],
    ).map_err(|_| AppError::InternalServerError)?;

    conn.execute(
        "
		CREATE TABLE IF NOT EXISTS p_certificates_meta (
			id INTEGER PRIMARY KEY AUTOINCREMENT,
			name TEXT NOT NULL,
			issuer TEXT NOT NULL,
			date TEXT NOT NULL,
			category TEXT NOT NULL
		)",
        &[],
    ).map_err(|_| AppError::InternalServerError)?;
    conn.execute(
        "
		CREATE TABLE IF NOT EXISTS p_certificates_images (
			id INTEGER PRIMARY KEY REFERENCES p_certificates_meta(id) ON DELETE CASCADE,
			image BLOB NOT NULL
		);
	",
        &[],
    ).map_err(|_| AppError::InternalServerError)?;
    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS p_resumes (
	    	id INTEGER PRIMARY KEY,
	     	doc BLOB NOT NULL,
			created_at INTEGER NOT NULL
		)
    ",
        &[],
    ).map_err(|_| AppError::InternalServerError)?;
    Ok(conn)
}

trait FromRow: Sized {
    fn from_row(row: Row) -> Result<Self>;
}
