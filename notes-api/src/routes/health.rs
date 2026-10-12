use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde_json::json;

use crate::app::AppState;

/// Readiness: "can I serve requests?" — checks the database.
/// Kubernetes stops sending traffic to this Pod while it returns 503.
#[utoipa::path(get, path = "/health", tag = "health",
    responses((status = 200, description = "Ready"), (status = 503, description = "Database unreachable")))]
pub async fn health_handler(State(state): State<AppState>) -> impl IntoResponse {
    match sqlx::query("SELECT 1").execute(&state.db).await {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({ "status": "ok", "database": "connected" })),
        ),
        Err(_) => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({ "status": "error", "database": "disconnected" })),
        ),
    }
}

/// Liveness: "is the process alive?" — never checks the database, so a database
/// outage doesn't make Kubernetes restart every Pod.
#[utoipa::path(get, path = "/health/live", tag = "health", responses((status = 200, description = "Alive")))]
pub async fn live_handler() -> impl IntoResponse {
    Json(json!({ "status": "alive" }))
}
