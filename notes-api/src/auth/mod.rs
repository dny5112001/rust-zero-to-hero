pub mod jwt;
pub mod password;

use axum::extract::FromRequestParts;
use axum::http::header::AUTHORIZATION;
use axum::http::request::Parts;
use uuid::Uuid;

use crate::app::AppState;
use crate::error::AppError;

/// Add `user: AuthUser` to a handler's arguments and it only runs for logged-in
/// users. Axum calls `from_request_parts` first; if it fails, the handler never runs.
pub struct AuthUser {
    pub id: Uuid,
}

#[axum::async_trait]
impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let header = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .ok_or(AppError::Unauthorized)?;
        let token = header
            .strip_prefix("Bearer ")
            .ok_or(AppError::Unauthorized)?;
        let id = jwt::verify_token(token, &state.config.jwt_secret)?;
        Ok(AuthUser { id })
    }
}
