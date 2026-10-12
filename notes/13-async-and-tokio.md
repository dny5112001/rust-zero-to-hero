# 13 · Async and Tokio

Example: [`basics/examples/13_async.rs`](../basics/examples/13_async.rs)

## The problem

A web server spends most of its time **waiting**: for the database, for the
network, for a file. With one thread per request, 10,000 users = 10,000
threads, mostly sitting idle and eating memory.

## The idea: a waiter with many tables

A restaurant waiter doesn't stand at one table until the food is cooked. They
take an order, **while it's cooking** they serve other tables, and come back
when it's ready.

**Async** works the same way: when a task has to wait, it **steps aside** so
the same thread can work on something else, and resumes when its data arrives.
One thread can juggle thousands of tasks.

## async and await

```rust
async fn fetch(name: &str, ms: u64) -> String {
    tokio::time::sleep(Duration::from_millis(ms)).await;  // wait without blocking the thread
    format!("{name} ({ms}ms)")
}
```

- `async fn` returns a **future** — a task that hasn't run yet.
- `.await` = "run this; if it has to wait, let others work meanwhile; continue when it's done".
- A future does **nothing** until it's awaited (like iterators are lazy).

## The runtime: Tokio

Rust's language has `async`/`await`, but something has to **schedule** the
tasks. That's a **runtime**, and **Tokio** is by far the most popular one.

```rust
#[tokio::main]           // starts the Tokio runtime and runs main inside it
async fn main() {
    let a = fetch("users", 300).await;
}
```

## One after another vs at the same time

```rust
// One after another: ~500ms
let a = fetch("users", 300).await;
let b = fetch("orders", 200).await;

// At the same time: ~300ms (only as long as the slowest)
let (a, b) = tokio::join!(fetch("users", 300), fetch("orders", 200));
```

Real output from the example:

```
sequential: users (300ms), orders (200ms) in 504ms
concurrent: users (300ms), orders (200ms) in 302ms
```

Other tools:

| Tool | Use |
|---|---|
| `tokio::spawn(async { ... })` | run a task in the background |
| `tokio::join!(a, b)` | wait for all |
| `tokio::select!` | wait for whichever finishes **first** (used for graceful shutdown in the notes API) |
| `tokio::time::timeout(dur, fut)` | give up after a while |
| `tokio::sync::{mpsc, Mutex, RwLock}` | async-friendly channels and locks |

## The one big rule: don't block

Inside async code, **never** do slow blocking work (heavy calculation,
`std::thread::sleep`, blocking file/network calls). It freezes the thread, and
every other task on it waits too — like the waiter stopping to cook.

- Waiting? Use the async version (`tokio::time::sleep`, `tokio::fs`, `sqlx`, `reqwest`).
- Heavy calculation? `tokio::task::spawn_blocking(|| ...)`.

> Fun fact: hashing a password with Argon2 is *deliberately* slow (~tens of
> milliseconds). A busy production API would move it into `spawn_blocking`.

## How the notes API uses it

```rust
#[tokio::main]
async fn main() {
    let pool = create_pool(&url).await.expect("db");   // connect to Postgres without blocking
    axum::serve(listener, app)                    // each request is an async task
        .with_graceful_shutdown(shutdown_signal())
        .await;
}
```

Every handler is an `async fn`, and every database query is `.await`ed — so one
small server handles many requests at once.

## Remember

- Async = don't sit idle while waiting; one thread juggles many tasks.
- `async fn` makes a future; `.await` runs it. Tokio is the runtime.
- `join!` to run things together; never block inside async code.
