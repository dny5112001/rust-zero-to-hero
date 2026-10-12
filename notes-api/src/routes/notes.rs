use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use uuid::Uuid;
use validator::Validate;

use crate::app::AppState;
use crate::auth::AuthUser;
use crate::error::{AppError, AppResult};
use crate::models::{Note, NoteInput};

// Every query filters by `user_id`, so users can only ever see their own notes.
// Someone else's note id gives 404, not 403 — we don't even confirm it exists.

const COLUMNS: &str = "id, user_id, title, body, created_at, updated_at";

#[utoipa::path(get, path = "/notes", tag = "notes", security(("bearer" = [])),
    responses((status = 200, body = [Note]), (status = 401)))]
pub async fn list_notes(
    State(state): State<AppState>,
    user: AuthUser,
) -> AppResult<Json<Vec<Note>>> {
    let notes = sqlx::query_as::<_, Note>(&format!(
        "SELECT {COLUMNS} FROM notes WHERE user_id = $1 ORDER BY updated_at DESC"
    ))
    .bind(user.id)
    .fetch_all(&state.db)
    .await?;
    Ok(Json(notes))
}

#[utoipa::path(post, path = "/notes", tag = "notes", security(("bearer" = [])), request_body = NoteInput,
    responses((status = 201, body = Note), (status = 401), (status = 422)))]
pub async fn create_note(
    State(state): State<AppState>,
    user: AuthUser,
    Json(input): Json<NoteInput>,
) -> AppResult<(StatusCode, Json<Note>)> {
    input.validate()?;
    let note = sqlx::query_as::<_, Note>(&format!(
        "INSERT INTO notes (id, user_id, title, body) VALUES ($1, $2, $3, $4) RETURNING {COLUMNS}"
    ))
    .bind(Uuid::new_v4())
    .bind(user.id)
    .bind(input.title.trim())
    .bind(&input.body)
    .fetch_one(&state.db)
    .await?;
    Ok((StatusCode::CREATED, Json(note)))
}

#[utoipa::path(get, path = "/notes/{id}", tag = "notes", security(("bearer" = [])),
    params(("id" = Uuid, Path)), responses((status = 200, body = Note), (status = 404)))]
pub async fn get_note(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
) -> AppResult<Json<Note>> {
    let note = sqlx::query_as::<_, Note>(&format!(
        "SELECT {COLUMNS} FROM notes WHERE id = $1 AND user_id = $2"
    ))
    .bind(id)
    .bind(user.id)
    .fetch_one(&state.db) // no row -> RowNotFound -> AppError::NotFound
    .await?;
    Ok(Json(note))
}

#[utoipa::path(put, path = "/notes/{id}", tag = "notes", security(("bearer" = [])),
    params(("id" = Uuid, Path)), request_body = NoteInput,
    responses((status = 200, body = Note), (status = 404), (status = 422)))]
pub async fn update_note(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
    Json(input): Json<NoteInput>,
) -> AppResult<Json<Note>> {
    input.validate()?;
    let note = sqlx::query_as::<_, Note>(&format!(
        "UPDATE notes SET title = $3, body = $4, updated_at = now()
         WHERE id = $1 AND user_id = $2 RETURNING {COLUMNS}"
    ))
    .bind(id)
    .bind(user.id)
    .bind(input.title.trim())
    .bind(&input.body)
    .fetch_one(&state.db)
    .await?;
    Ok(Json(note))
}

#[utoipa::path(delete, path = "/notes/{id}", tag = "notes", security(("bearer" = [])),
    params(("id" = Uuid, Path)), responses((status = 204), (status = 404)))]
pub async fn delete_note(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
) -> AppResult<StatusCode> {
    let result = sqlx::query("DELETE FROM notes WHERE id = $1 AND user_id = $2")
        .bind(id)
        .bind(user.id)
        .execute(&state.db)
        .await?;
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }
    Ok(StatusCode::NO_CONTENT)
}
