use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

/// A row in the `notes` table, and also what clients receive.
#[derive(Debug, Serialize, sqlx::FromRow, ToSchema)]
pub struct Note {
    pub id: Uuid,
    #[serde(skip)]
    pub user_id: Uuid,
    pub title: String,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Body for creating or updating a note.
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct NoteInput {
    #[validate(length(min = 1, max = 200, message = "must be 1-200 characters"))]
    #[schema(example = "Shopping list")]
    pub title: String,
    #[serde(default)]
    #[validate(length(max = 10000, message = "must be at most 10000 characters"))]
    #[schema(example = "Milk, eggs, bread")]
    pub body: String,
}
