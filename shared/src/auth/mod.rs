use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use spin_sdk::variables;

use crate::{
    db::user::Role,
    errors::{AppError, Result},
};

pub fn get_secret() -> Result<Vec<u8>> {
    Ok(variables::get("jwt_secret")
        .map_err(|e| AppError::ISE(e.to_string().into_boxed_str()))?
        .as_bytes()
        .to_owned())
}

pub mod extractor;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
    pub role: Role,
}
impl Claims {
    pub fn new(user_id: &str, role: Role) -> Self {
        let now = chrono::Utc::now().timestamp();

        Self {
            sub: user_id.to_string(),
            role,
            iat: now,
            exp: (now + chrono::Duration::hours(24).num_milliseconds()),
        }
    }
}

pub fn create_token(user_id: &str, role: Role, secret: &[u8]) -> Result<String> {
    let now = chrono::Utc::now().timestamp();

    let claims = Claims {
        sub: user_id.to_string(),
        role,
        iat: now,
        exp: (now + chrono::Duration::hours(24).num_milliseconds()),
    };

    Ok(encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret),
    ).map_err(|e| AppError::ISE(e.to_string().into_boxed_str()))?)
}

pub fn verify_token(token: &str, secret: &[u8]) -> Result<Claims> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret),
        &Validation::new(Algorithm::HS256),
    ).map_err(|e| AppError::ISE(e.to_string().into_boxed_str()))?;
    Ok(data.claims)
}

pub fn hash_password(password: &str) -> Result<String> {
    let argon2 = Argon2::default();

    let hash = argon2
        .hash_password(password.as_bytes())
        .map_err(|e| AppError::ISE(e.to_string().into_boxed_str()))?
        .to_string();

    Ok(hash)
}

pub fn verify_password(password: &str, stored_hash: &str) -> Result<bool> {
    let parsed_hash =
        PasswordHash::new(stored_hash).map_err(|e| AppError::ISE(e.to_string().into_boxed_str()))?;

    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}
