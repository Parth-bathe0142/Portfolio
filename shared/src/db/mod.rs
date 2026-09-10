use anyhow::Result;
use spin_sdk::sqlite::{Connection, Row};

pub mod certificates;
pub mod user;

pub use certificates::CertificateMeta;
pub use user::User;

pub fn get_connection() -> Result<Connection> {
    let conn = Connection::open_default()?;

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
    )?;

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
    )?;
    conn.execute(
        "
		CREATE TABLE IF NOT EXISTS p_certificates_images (
			id INTEGER PRIMARY KEY REFERENCES p_certificates_meta(id) ON DELETE CASCADE,
			image BLOB NOT NULL
		);
	",
        &[],
    )?;
    Ok(conn)
}

trait FromRow: Sized {
    fn from_row(row: Row) -> Result<Self>;
}
