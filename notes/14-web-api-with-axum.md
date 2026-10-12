# 14 · Building a web API with Axum

From here on we work in [`notes-api/`](../notes-api) — the notes service you
started, now finished. It lets users sign up, log in, and keep private notes.

## Run it

```bash
cd notes-api
cp .env.example .env
docker compose up -d db        # Postgres in Docker on port 5433
cargo run                      # the API on http://localhost:8810
```

Open **http://localhost:8810/docs** — interactive docs where you can try every endpoint.

## The endpoints

| Method + path | Needs login? | Does |
|---|---|---|
| `GET /health` | no | ready? (checks the database) |
| `GET /health/live` | no | alive? (process only) |
| `POST /auth/register` | no | create an account |
| `POST /auth/login` | no | get a login token (JWT) |
| `GET /notes` | yes | list my notes |
| `POST /notes` | yes | create a note |
| `GET /notes/:id` | yes | read one |
| `PUT /notes/:id` | yes | change one |
| `DELETE /notes/:id` | yes | delete one |

## How a request flows

```
HTTP request
   │
   ▼
Router (app.rs)            "POST /notes" → notes::create_note
   │
   ▼
Extractors                 State, AuthUser (checks the token), Json<NoteInput> (parses the body)
   │   (if any fails → error response, handler never runs)
   ▼
Handler (routes/notes.rs)  validate → query the database → return a value
   │
   ▼
IntoResponse               value → HTTP response (status + JSON)
```

## The router

[`src/app.rs`](../notes-api/src/app.rs):

```rust
pub fn create_app(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health::health_handler))
        .route("/notes", get(notes::list_notes).post(notes::create_note))
        .route("/notes/:id", get(notes::get_note).put(notes::update_note).delete(notes::delete_note))
        .layer(TraceLayer::new_for_http())     // log every request
        .with_state(state)                     // shared things every handler can use
}
```

`AppState` holds the database pool and config. It's cloned per request — cheap,
because the pool is shared inside (`Arc`).

## Handlers and extractors

A handler is just an `async fn`. Its **arguments** say what it needs, and Axum
fills them in — these are **extractors**:

```rust
pub async fn create_note(
    State(state): State<AppState>,     // the shared state
    user: AuthUser,                    // the logged-in user (from the token)
    Json(input): Json<NoteInput>,      // the request body, parsed from JSON
) -> AppResult<(StatusCode, Json<Note>)> {
    input.validate()?;
    let note = /* insert into the database */;
    Ok((StatusCode::CREATED, Json(note)))
}
```

Other extractors: `Path(id)` (from `/notes/:id`), `Query(params)` (from `?page=2`), `HeaderMap`.

The **return value** just has to implement `IntoResponse`: `Json<T>`,
`StatusCode`, a `(StatusCode, Json<T>)` tuple, or a `Result` of those.

## JSON with serde

```rust
#[derive(Deserialize, Validate)]        // JSON → Rust (requests)
pub struct NoteInput {
    #[validate(length(min = 1, max = 200))]
    pub title: String,
    #[serde(default)]                  // missing in the JSON? use "" instead
    pub body: String,
}

#[derive(Serialize)]                    // Rust → JSON (responses)
pub struct Note {
    pub id: Uuid,
    #[serde(skip)]                     // never sent to clients
    pub user_id: Uuid,
    pub title: String,
    // ...
}
```

`serde` turns structs into JSON and back. Bad JSON never reaches your handler —
Axum answers `400`/`422` for you.

## Validation

The `validator` crate checks input rules declared on the struct
(`#[validate(email)]`, `length(min = 8)`), and `input.validate()?` turns
failures into a `422 Unprocessable Entity` with a readable message.

## Logging

`tracing` writes structured logs; `RUST_LOG` controls how much:

```bash
RUST_LOG=notes_api=debug,tower_http=info cargo run
```

```rust
tracing::info!(user_id = %user.id, "user registered");
```

## API docs (OpenAPI / Swagger)

`utoipa` reads `#[utoipa::path(...)]` notes on each handler and builds an
OpenAPI description, served with Swagger UI at `/docs`. We enable its
`vendored` feature so the Swagger files are bundled in the crate — the build
doesn't need to download anything (this mattered for the Docker build in chapter 18).

## Try it

```bash
B=http://localhost:8810
curl -X POST $B/auth/register -H 'content-type: application/json' -d '{"email":"me@example.com","password":"a-long-password"}'
TOKEN=$(curl -s -X POST $B/auth/login -H 'content-type: application/json' -d '{"email":"me@example.com","password":"a-long-password"}' | jq -r .access_token)
curl -X POST $B/notes -H "authorization: Bearer $TOKEN" -H 'content-type: application/json' -d '{"title":"First note"}'
curl $B/notes -H "authorization: Bearer $TOKEN"
```

Then add an endpoint: `GET /notes/count` returning `{"count": n}`.

## Remember

- Router → extractors → handler → `IntoResponse`.
- Handler arguments are extractors; if one fails, the handler never runs.
- serde for JSON, validator for rules, tracing for logs, utoipa for docs.
