use axum::extract::FromRef;
use axum_extra::extract::cookie::Key;
use spin_sdk::variables;

use crate::errors::Result;

#[derive(Clone)]
pub struct AppState {
    pub key: Key,
}
impl AppState {
	pub fn new() -> Result<Self> {
        Ok(Self {
            key: Key::from(
                variables::get("jwt_secret")
                    .expect("JWT secret not set")
                    .as_bytes(),
            ),
        })
    }
}

impl FromRef<AppState> for Key {
	fn from_ref(input: &AppState) -> Self {
    	input.key.clone()
	}
}