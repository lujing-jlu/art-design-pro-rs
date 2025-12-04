use crate::error::AppError;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;
use std::{
    env,
    time::{SystemTime, UNIX_EPOCH},
};

pub fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| AppError::InternalServerError(e.to_string()))?
        .to_string();
    Ok(password_hash)
}

pub fn verify_password(password: &str, password_hash: &str) -> Result<bool, AppError> {
    let parsed_hash = PasswordHash::new(password_hash)
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;
    let argon2 = Argon2::default();
    Ok(argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String, // Subject (username)
    pub uid: i64,    // User ID
    pub exp: usize,  // Expiration time
}

pub fn generate_token(username: &str, uid: i64) -> Result<String, AppError> {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize
        + get_token_expiration_secs();

    let claims = Claims {
        sub: username.to_owned(),
        uid,
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(get_secret_key()),
    )
    .map_err(|e| AppError::InternalServerError(e.to_string()))
}

pub fn verify_token(token: &str) -> Result<Claims, AppError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(get_secret_key()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| AppError::AuthError(e.to_string()))
}

/// 获取 JWT 密钥，优先使用环境变量 JWT_SECRET，否则回退到默认值
fn get_secret_key() -> &'static [u8] {
    static SECRET: OnceLock<Vec<u8>> = OnceLock::new();
    SECRET
        .get_or_init(|| {
            env::var("JWT_SECRET")
                .unwrap_or_else(|_| "change_me_secret".to_string())
                .into_bytes()
        })
        .as_slice()
}

fn get_token_expiration_secs() -> usize {
    static EXPIRE: OnceLock<usize> = OnceLock::new();
    *EXPIRE.get_or_init(|| {
        env::var("JWT_EXPIRE_SECS")
            .ok()
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(86_400)
    })
}
