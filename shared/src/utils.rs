use anyhow::Result;
use axum_extra::extract::cookie::{Cookie, SameSite};
use base64::prelude::{BASE64_STANDARD, Engine};
use spin_sdk::{
    sqlite::Value,
};

pub fn int(num: i64) -> Value {
    Value::Integer(num)
}

pub fn text(s: String) -> Value {
    Value::Text(s)
}

pub fn blob(b: Vec<u8>) -> Value {
    Value::Blob(b)
}

pub fn decode_base64(s: &str) -> Result<Vec<u8>> {
    BASE64_STANDARD.decode(s).map_err(|e| e.into())
}

pub fn hard_cookie(name: &str, value: String) -> Cookie<'_> {
	Cookie::build((name, value))
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .path("/")
        .build()
}