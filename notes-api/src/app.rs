use axum::{Router, routing::get, routing::post};
use sqlx::PgPool;
use tower_http::trace::TraceLayer;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{Modify, OpenApi};
use utoipa_swagger_ui::SwaggerUi;

use crate::config::AppConfig;
use crate::models::{Credentials, Note, NoteInput, TokenResponse, UserResponse};
use crate::routes::{auth, health, notes};

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub config: AppConfig,
}

pub fn create_app(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health::health_handler))
        .route("/health/live", get(health::live_handler))
        .route("/auth/register", post(auth::register))
        .route("/auth/login", post(auth::login))
        .route("/notes", get(notes::list_notes).post(notes::create_note))
        .route(
            "/notes/:id",
            get(notes::get_note)
                .put(notes::update_note)
                .delete(notes::delete_note),
        )
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(TraceLayer::new_for_http()) // logs every request
        .with_state(state)
}

/// The OpenAPI description, served as interactive docs at /docs.
#[derive(OpenApi)]
#[openapi(
    info(title = "Notes API", description = "A small notes service written in Rust"),
    paths(
        health::health_handler, health::live_handler,
        auth::register, auth::login,
        notes::list_notes, notes::create_note, notes::get_note, notes::update_note, notes::delete_note,
    ),
    components(schemas(Credentials, UserResponse, TokenResponse, Note, NoteInput)),
    modifiers(&BearerAuth),
)]
pub struct ApiDoc;

/// Adds the "Authorize" button in Swagger UI for the JWT.
struct BearerAuth;

impl Modify for BearerAuth {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.get_or_insert_with(Default::default);
        components.add_security_scheme(
            "bearer",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );
    }
}
