use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

/// A row in the `users` table. Never sent to clients (it holds the password hash).
#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}

/// Body for /auth/register and /auth/login.
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct Credentials {
    #[validate(email(message = "must be a valid email address"))]
    #[schema(example = "asha@example.com")]
    pub email: String,
    #[validate(length(min = 8, message = "must be at least 8 characters"))]
    #[schema(example = "a-long-password")]
    pub password: String,
}

/// What clients see about a user.
#[derive(Debug, Serialize, ToSchema)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            id: user.id,
            email: user.email,
            created_at: user.created_at,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: &'static str,
    pub expires_in: i64,
}
