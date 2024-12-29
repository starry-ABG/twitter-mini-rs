use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use anyhow::{Result, anyhow};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // 可以放 user_id 或 username
    pub exp: usize,   // 过期时间戳
}

pub fn generate_token(secret: &str, expire_hours: i64, subject: &str) -> Result<String> {
    let now = Utc::now();
    let exp = now
        .checked_add_signed(Duration::hours(expire_hours))
        .ok_or_else(|| anyhow!("Invalid exp"))?
        .timestamp();

    let claims = Claims {
        sub: subject.to_owned(),
        exp: exp as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )?;
    Ok(token)
}

pub fn verify_token(secret: &str, token: &str) -> Result<Claims> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    )?;
    Ok(data.claims)
}