use anyhow::{Result, bail};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use spin_sdk::{http::Request, variables};

use crate::{db::user::Role, utils::get_cookies};

pub fn get_secret() -> Result<Vec<u8>> {
    Ok(variables::get("jwt_secret")?.as_bytes().to_owned())
}

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

pub fn create_token(user_id: &str, role: Role, secret: &[u8]) -> anyhow::Result<String> {
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
    )?)
}

pub fn verify_token(token: &str, secret: &[u8]) -> anyhow::Result<Claims> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret),
        &Validation::new(Algorithm::HS256),
    )?;
    Ok(data.claims)
}

pub fn authenticate(req: Request, role: Role) -> Result<()> {
    let Some(cookies) = get_cookies(&req) else {
        bail!("missing cookies");
    };

    let Some(token) = cookies.get("jwt_token") else {
        bail!("not authenticated");
    };

    let claims = verify_token(token, &get_secret()?)?;

    if claims.role == role {
        Ok(())
    } else {
        bail!("not autherized")
    }
}

pub fn hash_password(password: &str) -> anyhow::Result<String> {
    let argon2 = Argon2::default();

    let hash = argon2
        .hash_password(password.as_bytes())
        .map_err(|e| anyhow::anyhow!("hashing failed: {}", e))?
        .to_string();

    Ok(hash)
}

pub fn verify_password(password: &str, stored_hash: &str) -> anyhow::Result<bool> {
    let parsed_hash = PasswordHash::new(stored_hash)
        .map_err(|e| anyhow::anyhow!("invalid stored hash: {}", e))?;

    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}
