# 15 · Talking to Postgres with SQLx

## Why SQLx

**SQLx** is an async database library. You write **real SQL** (no hidden
query builder), and it maps rows into your structs.

## The connection pool

Opening a database connection is slow, so we keep a few open and share them —
a **pool**. [`src/db/postgres.rs`](../notes-api/src/db/postgres.rs):

```rust
PgPoolOptions::new()
    .max_connections(5)
    .connect(database_url)
    .await
```

Each request borrows a connection, uses it, and gives it back automatically.

## Migrations — versioned schema changes

The tables are defined in numbered SQL files in [`migrations/`](../notes-api/migrations):

```
migrations/
├── 20260101000001_create_users.sql
└── 20260101000002_create_notes.sql
```

```sql
CREATE TABLE notes (
    id         UUID PRIMARY KEY,
    user_id    UUID NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    title      TEXT NOT NULL,
    body       TEXT NOT NULL DEFAULT '',
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
```

At start-up, `main.rs` runs any that haven't run yet:

```rust
sqlx::migrate!("./migrations").run(&pool).await
```

`migrate!` **embeds** the files into the binary at compile time, so the Docker
image doesn't need the folder. SQLx records what ran in a `_sqlx_migrations`
table, so each runs exactly once.

**Rules:** never edit a migration that has already run somewhere — add a new
one instead. To change the schema: create `migrations/<timestamp>_add_tags.sql`
(the `sqlx-cli` tool can do it: `cargo install sqlx-cli`, then `sqlx migrate add add_tags`).

## Queries

```rust
let note = sqlx::query_as::<_, Note>(
    "SELECT id, user_id, title, body, created_at, updated_at
     FROM notes WHERE id = $1 AND user_id = $2",
)
.bind(id)               // fills $1
.bind(user.id)          // fills $2
.fetch_one(&state.db)   // exactly one row, or an error
.await?;
```

- `query_as::<_, Note>` maps each row into `Note` (which has `#[derive(sqlx::FromRow)]`).
- `$1, $2` are **placeholders**; values go in with `.bind()`. **Never** build SQL
  by pasting user input into the string — that's how SQL injection happens.
  Placeholders make it impossible.

| Method | Returns |
|---|---|
| `fetch_one` | one row (error `RowNotFound` if none) |
| `fetch_optional` | `Option<row>` |
| `fetch_all` | `Vec<row>` |
| `execute` | how many rows were affected (for UPDATE/DELETE) |

### Turning "not found" into a 404

In [`src/error.rs`](../notes-api/src/error.rs):

```rust
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => AppError::NotFound,      // -> 404
            other => AppError::Internal(other.to_string()),       // -> 500, details only in logs
        }
    }
}
```

So `fetch_one(...).await?` in a handler becomes a clean 404 automatically.

### Safety by design: every query filters by user

```sql
... WHERE id = $1 AND user_id = $2
```

Asking for someone else's note finds **no row** → 404. One user can never read,
change or delete another user's notes — and there's an integration test proving it.

### Handling duplicates in SQL

Registration uses `ON CONFLICT (email) DO NOTHING RETURNING ...` with
`fetch_optional`: no row back means the email already existed → `409 Conflict`.
Doing it in one statement avoids a race between "check" and "insert".

## Compile-time checked queries (optional)

SQLx can check your SQL against a real database **while compiling**, with the
`query!` / `query_as!` macros. Great safety, but builds then need a database (or
a prepared `.sqlx` folder via `cargo sqlx prepare`). We use the plain functions
to keep Docker builds simple.

## Try it

1. Add a migration that adds `pinned BOOLEAN NOT NULL DEFAULT false` to `notes`, add the field to `Note`, and sort pinned notes first.
2. Connect with `docker compose exec db psql -U notes notes_db` and run `\d notes` and `SELECT * FROM _sqlx_migrations;`.

## Remember

- A pool shares a few connections; migrations version your schema and run at start-up.
- Always use `$1` placeholders + `.bind()` — never paste input into SQL.
- Map database errors to HTTP errors in one place (`From<sqlx::Error>`).
