//! Integration tests: the real app against a real Postgres.
//!
//! `#[sqlx::test]` creates a brand-new empty database for EACH test, runs the
//! migrations, and deletes it afterwards. It needs DATABASE_URL pointing at a
//! Postgres server (e.g. `docker compose up -d db` and the value in .env.example).

mod common;

use axum::http::StatusCode;
use common::{register_and_login, send, test_app};
use serde_json::json;
use sqlx::PgPool;

#[sqlx::test]
async fn health_reports_database_connected(pool: PgPool) {
    let app = test_app(pool);
    let (status, body) = send(&app, "GET", "/health", None, None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["database"], "connected");
}

#[sqlx::test]
async fn register_rejects_bad_input_and_duplicates(pool: PgPool) {
    let app = test_app(pool);

    let bad = json!({ "email": "not-an-email", "password": "short" });
    let (status, _) = send(&app, "POST", "/auth/register", None, Some(bad)).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);

    let good = json!({ "email": "Asha@Example.com", "password": "a-long-password" });
    let (status, body) = send(&app, "POST", "/auth/register", None, Some(good.clone())).await;
    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(body["email"], "asha@example.com"); // stored lower-case
    assert!(body.get("password_hash").is_none()); // never leaked

    let (status, _) = send(&app, "POST", "/auth/register", None, Some(good)).await;
    assert_eq!(status, StatusCode::CONFLICT);
}

#[sqlx::test]
async fn login_with_wrong_password_fails(pool: PgPool) {
    let app = test_app(pool);
    register_and_login(&app, "ravi@example.com").await;

    let wrong = json!({ "email": "ravi@example.com", "password": "not-the-password" });
    let (status, _) = send(&app, "POST", "/auth/login", None, Some(wrong)).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

#[sqlx::test]
async fn notes_need_a_token(pool: PgPool) {
    let app = test_app(pool);
    let (status, _) = send(&app, "GET", "/notes", None, None).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    let (status, _) = send(&app, "GET", "/notes", Some("garbage"), None).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

#[sqlx::test]
async fn full_note_lifecycle(pool: PgPool) {
    let app = test_app(pool);
    let token = register_and_login(&app, "meera@example.com").await;
    let t = Some(token.as_str());

    // create
    let (status, note) = send(
        &app,
        "POST",
        "/notes",
        t,
        Some(json!({ "title": "Groceries", "body": "milk" })),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    let id = note["id"].as_str().unwrap().to_string();
    let url = format!("/notes/{id}");

    // read
    let (status, fetched) = send(&app, "GET", &url, t, None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(fetched["title"], "Groceries");

    // update
    let (status, updated) = send(
        &app,
        "PUT",
        &url,
        t,
        Some(json!({ "title": "Groceries", "body": "milk, eggs" })),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(updated["body"], "milk, eggs");

    // list
    let (_, list) = send(&app, "GET", "/notes", t, None).await;
    assert_eq!(list.as_array().unwrap().len(), 1);

    // delete, then it's gone
    let (status, _) = send(&app, "DELETE", &url, t, None).await;
    assert_eq!(status, StatusCode::NO_CONTENT);
    let (status, _) = send(&app, "GET", &url, t, None).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[sqlx::test]
async fn users_cannot_see_each_others_notes(pool: PgPool) {
    let app = test_app(pool);
    let alice = register_and_login(&app, "alice@example.com").await;
    let bob = register_and_login(&app, "bob@example.com").await;

    let (_, note) = send(
        &app,
        "POST",
        "/notes",
        Some(&alice),
        Some(json!({ "title": "Alice's secret" })),
    )
    .await;
    let url = format!("/notes/{}", note["id"].as_str().unwrap());

    let (status, _) = send(&app, "GET", &url, Some(&bob), None).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    let (status, _) = send(&app, "DELETE", &url, Some(&bob), None).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    let (_, bobs_list) = send(&app, "GET", "/notes", Some(&bob), None).await;
    assert!(bobs_list.as_array().unwrap().is_empty());
}

#[sqlx::test]
async fn empty_title_is_rejected(pool: PgPool) {
    let app = test_app(pool);
    let token = register_and_login(&app, "kiran@example.com").await;
    let (status, body) = send(
        &app,
        "POST",
        "/notes",
        Some(&token),
        Some(json!({ "title": "" })),
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    assert!(body["error"].as_str().unwrap().contains("title"));
}
