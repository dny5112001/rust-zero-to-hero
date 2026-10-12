use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::AppError;

/// What we put inside a login token.
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// "subject": the user's id
    pub sub: Uuid,
    /// "expires at", in seconds since 1970
    pub exp: i64,
    /// "issued at"
    pub iat: i64,
}

pub fn create_token(user_id: Uuid, secret: &str, ttl_seconds: i64) -> Result<String, AppError> {
    let now = Utc::now().timestamp();
    let claims = Claims {
        sub: user_id,
        iat: now,
        exp: now + ttl_seconds,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AppError::Internal(e.to_string()))
}

/// Check the signature and expiry, and return the user id inside.
pub fn verify_token(token: &str, secret: &str) -> Result<Uuid, AppError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims.sub)
    .map_err(|_| AppError::Unauthorized)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_round_trip() {
        let id = Uuid::new_v4();
        let token = create_token(id, "secret", 60).unwrap();
        assert_eq!(verify_token(&token, "secret").unwrap(), id);
    }

    #[test]
    fn wrong_secret_is_rejected() {
        let token = create_token(Uuid::new_v4(), "secret", 60).unwrap();
        assert!(verify_token(&token, "other-secret").is_err());
    }

    #[test]
    fn expired_token_is_rejected() {
        // Expired well beyond the library's default 60s leeway.
        let token = create_token(Uuid::new_v4(), "secret", -120).unwrap();
        assert!(verify_token(&token, "secret").is_err());
    }
}
