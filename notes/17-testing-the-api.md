# 17 · Testing the API

```bash
cd notes-api
docker compose up -d db
DATABASE_URL=postgres://notes:notes@localhost:5433/notes_db cargo test
```

```
test auth::jwt::tests::token_round_trip ... ok
test auth::jwt::tests::wrong_secret_is_rejected ... ok
test auth::jwt::tests::expired_token_is_rejected ... ok
test auth::password::tests::right_password_verifies_and_wrong_one_does_not ... ok
test auth::password::tests::same_password_gets_different_hashes ... ok
test health_reports_database_connected ... ok
test register_rejects_bad_input_and_duplicates ... ok
test login_with_wrong_password_fails ... ok
test notes_need_a_token ... ok
test full_note_lifecycle ... ok
test users_cannot_see_each_others_notes ... ok
test empty_title_is_rejected ... ok
```

## Two layers

| Layer | Where | Tests |
|---|---|---|
| **Unit** | inside `src/auth/*.rs` | small pieces with no database: hashing, tokens |
| **Integration** | `tests/api.rs` | the real app + a real Postgres, through HTTP requests |

## A fresh database for every test

```rust
#[sqlx::test]
async fn full_note_lifecycle(pool: PgPool) {
    let app = test_app(pool);
    // ...
}
```

`#[sqlx::test]` creates a **brand-new empty database** for each test, runs the
migrations, hands you a pool, and deletes the database afterwards. Tests can't
interfere with each other and can run in parallel. It needs `DATABASE_URL`
pointing at a server where it may create databases.

## Calling the app without a network

[`tests/common/mod.rs`](../notes-api/tests/common/mod.rs) builds the real router
and sends requests straight into it with `tower::ServiceExt::oneshot` — no port,
no HTTP client, very fast:

```rust
let res = app.clone().oneshot(request).await.unwrap();
```

Helpers keep tests readable:

```rust
let token = register_and_login(&app, "alice@example.com").await;
let (status, note) = send(&app, "POST", "/notes", Some(&token), Some(json!({ "title": "Hi" }))).await;
assert_eq!(status, StatusCode::CREATED);
```

## Test what matters

The tests focus on **behaviour and security**, not on lines of code:

- bad input is rejected (422), duplicates are refused (409),
- wrong password → 401, no token or garbage token → 401,
- the full create → read → update → list → delete → 404 cycle,
- **user B cannot read or delete user A's note** (the most important one).

## Quality checks

```bash
cargo fmt --check        # formatting
cargo clippy --all-targets -- -D warnings   # lints, fail on any warning
cargo test
```

Run all three in CI on every pull request.

## Try it

1. Add a test: updating a note with an empty title returns 422.
2. Break the `AND user_id = $2` filter in `get_note` and watch `users_cannot_see_each_others_notes` fail. Put it back!

## Remember

- Unit tests for pure logic; integration tests for the real app + real database.
- `#[sqlx::test]` = a fresh database per test.
- Test security rules explicitly.
