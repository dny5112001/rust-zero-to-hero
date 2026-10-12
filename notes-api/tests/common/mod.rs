//! Helpers shared by the integration tests.

use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use notes_api::app::{AppState, create_app};
use notes_api::config::AppConfig;
use serde_json::{Value, json};
use sqlx::PgPool;
use tower::ServiceExt; // for `oneshot`

pub fn test_app(pool: PgPool) -> Router {
    let config = AppConfig {
        host: "127.0.0.1".into(),
        port: 0,
        jwt_secret: "test-secret".into(),
        jwt_ttl_seconds: 3600,
    };
    create_app(AppState { db: pool, config })
}

/// Send one request to the app (no real network) and return status + JSON body.
pub async fn send(
    app: &Router,
    method: &str,
    uri: &str,
    token: Option<&str>,
    body: Option<Value>,
) -> (StatusCode, Value) {
    let mut req = Request::builder().method(method).uri(uri);
    if let Some(token) = token {
        req = req.header("authorization", format!("Bearer {token}"));
    }
    let req = match body {
        Some(json) => req
            .header("content-type", "application/json")
            .body(Body::from(json.to_string())),
        None => req.body(Body::empty()),
    }
    .unwrap();

    let res = app.clone().oneshot(req).await.unwrap();
    let status = res.status();
    let bytes = res.into_body().collect().await.unwrap().to_bytes();
    let value = serde_json::from_slice(&bytes).unwrap_or(Value::Null);
    (status, value)
}

/// Register a user and log in; returns their token.
pub async fn register_and_login(app: &Router, email: &str) -> String {
    let creds = json!({ "email": email, "password": "a-long-password" });
    let (status, _) = send(app, "POST", "/auth/register", None, Some(creds.clone())).await;
    assert_eq!(status, StatusCode::CREATED);
    let (status, body) = send(app, "POST", "/auth/login", None, Some(creds)).await;
    assert_eq!(status, StatusCode::OK);
    body["access_token"].as_str().unwrap().to_string()
}
