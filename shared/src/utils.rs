use std::collections::HashMap;

use anyhow::Result;
use askama::Template;
use base64::prelude::{BASE64_STANDARD, Engine};
use http::StatusCode;
use spin_sdk::{
    http::{Request, Response, ResponseBuilder},
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

pub fn templ(templ: impl Template) -> Result<Response> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(templ.render()?)
        .build())
}

pub fn parse_body(req: &Request) -> HashMap<String, String> {
    url::form_urlencoded::parse(req.body())
        .into_owned()
        .collect()
}

pub fn decode_base64(s: &str) -> Result<Vec<u8>> {
    BASE64_STANDARD.decode(s).map_err(|e| e.into())
}

pub fn redirect(url: &str, htmx: bool) -> Response {
    let (code, head) = if htmx {
        (StatusCode::OK, "HX-Redirect")
    } else {
        (StatusCode::FOUND, "Location")
    };

    Response::builder().status(code).header(head, url).build()
}

pub fn add_cookie(
    mut builder: ResponseBuilder,
    name: &str,
    value: &str,
    max_age: i64,
    path: &str,
) -> ResponseBuilder {
    let cookie =
        format!("{name}={value}; HttpOnly; SameSite=Strict; Path={path}; Max-Age={max_age}");
    builder.header("Set-Cookie", cookie).build().into_builder()
}

pub fn get_cookies(req: &Request) -> Option<HashMap<String, String>> {
    let header_val = req.header("cookie")?.as_str()?;

    Some(
        header_val
            .split(';')
            .map(|kv| kv.trim().split_once('=').unwrap())
            .map(|(a, b)| (a.to_owned(), b.to_owned()))
            .collect(),
    )
}
