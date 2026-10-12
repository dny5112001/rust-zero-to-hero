use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use uuid::Uuid;
use validator::Validate;

use crate::app::AppState;
use crate::auth::{jwt, password};
use crate::error::{AppError, AppResult};
use crate::models::{Credentials, TokenResponse, User, UserResponse};

#[utoipa::path(post, path = "/auth/register", tag = "auth", request_body = Credentials,
    responses((status = 201, body = UserResponse), (status = 409, description = "Email already registered"),
              (status = 422, description = "Invalid input")))]
pub async fn register(
    State(state): State<AppState>,
    Json(input): Json<Credentials>,
) -> AppResult<(StatusCode, Json<UserResponse>)> {
    input.validate()?;
    let email = input.email.trim().to_lowercase();
    let hash = password::hash_password(&input.password)?;

    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (id, email, password_hash) VALUES ($1, $2, $3)
         ON CONFLICT (email) DO NOTHING
         RETURNING id, email, password_hash, created_at",
    )
    .bind(Uuid::new_v4())
    .bind(&email)
    .bind(hash)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::Conflict("email is already registered".into()))?;

    tracing::info!(user_id = %user.id, "user registered");
    Ok((StatusCode::CREATED, Json(user.into())))
}

#[utoipa::path(post, path = "/auth/login", tag = "auth", request_body = Credentials,
    responses((status = 200, body = TokenResponse), (status = 401, description = "Wrong email or password")))]
pub async fn login(
    State(state): State<AppState>,
    Json(input): Json<Credentials>,
) -> AppResult<Json<TokenResponse>> {
    let email = input.email.trim().to_lowercase();
    let user = sqlx::query_as::<_, User>(
        "SELECT id, email, password_hash, created_at FROM users WHERE email = $1",
    )
    .bind(&email)
    .fetch_optional(&state.db)
    .await?;

    // Same error for "no such user" and "wrong password", so attackers can't
    // find out which emails are registered.
    let user = match user {
        Some(user) if password::verify_password(&input.password, &user.password_hash) => user,
        _ => return Err(AppError::InvalidCredentials),
    };

    let token = jwt::create_token(
        user.id,
        &state.config.jwt_secret,
        state.config.jwt_ttl_seconds,
    )?;
    Ok(Json(TokenResponse {
        access_token: token,
        token_type: "Bearer",
        expires_in: state.config.jwt_ttl_seconds,
    }))
}
