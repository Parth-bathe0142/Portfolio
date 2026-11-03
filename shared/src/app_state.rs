use anyhow::{Result, anyhow};
use axum::extract::FromRef;
use axum_extra::extract::cookie::Key;
use spin_sdk::variables;

#[derive(Clone)]
pub struct AppState {
    key: Key,
}
impl AppState {
	pub fn new() -> Result<Self> {
        Ok(Self {
            key: Key::from(
                variables::get("jwt_secret")
                    .map_err(|_| anyhow!("secret variable jwt_secret not set"))?
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