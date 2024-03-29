use super::{claims::Claims, errors::JWTError};
use argon2::Argon2;

pub(super) fn create_refresh_token(claims: &Claims) -> Result<String, JWTError> {
    let iat = claims.iat.unix_timestamp();
    let exp = claims.iat.unix_timestamp();
    let uuid: &str = &claims.uuid;

    let iat = hex::encode(iat.to_be_bytes());
    let exp = hex::encode(exp.to_be_bytes());

    let ref_token_raw = String::from
        (iat + &exp + uuid)
        .into_bytes();

    let salt = &claims.email.as_bytes();

    let hasher = Argon2::default();

    let mut refresh_token = [0u8; 32];
    hasher.hash_password_into(&ref_token_raw, &salt, &mut refresh_token)
        .map_err(|e| JWTError::custom(&e.to_string()))?;

    Ok(hex::encode(refresh_token))
}
