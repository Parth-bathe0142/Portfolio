use axum_extra::extract::cookie::{Cookie, SameSite};
use spin_sdk::sqlite::{Connection, Value};

use crate::errors::{AppError, Result};

pub fn int(num: i64) -> Value {
    Value::Integer(num)
}

pub fn text(s: String) -> Value {
    Value::Text(s)
}

pub fn blob(b: Vec<u8>) -> Value {
    Value::Blob(b)
}

pub fn hard_cookie(name: &str, value: String) -> Cookie<'_> {
    Cookie::build((name, value))
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .path("/")
        .build()
}

pub fn last_rowid(conn: &Connection) -> Result<i64> {
    Ok(conn
        .execute("SELECT last_insert_rowid()", &[])
        .map_err(|_| AppError::ISE("db error".into()))?
        .rows
        .first()
        .unwrap()
        .get::<i64>(0)
        .unwrap())
}
