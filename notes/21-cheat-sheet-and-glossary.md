# 21 · Cheat sheet, glossary and interview questions

## Cargo

```bash
cargo new app / cargo new --lib mylib
cargo run [--example name] [--release]
cargo check · cargo build --release · cargo test [name] · cargo fmt · cargo clippy
cargo add crate [--features x] · cargo update · cargo doc --open · cargo tree
```

## Syntax on one page

```rust
let x = 5;  let mut y = 0;  const MAX: u32 = 100;
fn add(a: i32, b: i32) -> i32 { a + b }
let s = if x > 3 { "big" } else { "small" };
for i in 0..10 {}  while cond {}  let v = loop { break 1; };

struct User { name: String, age: u32 }
impl User { fn new(name: &str) -> Self { User { name: name.into(), age: 0 } } fn greet(&self) {} }
enum Shape { Circle(f64), Square { side: f64 } }
match shape { Shape::Circle(r) => r, Shape::Square { side } => side }

let o: Option<i32> = Some(1);   if let Some(v) = o {}   o.unwrap_or(0)
fn f() -> Result<i32, MyError> { let n = "5".parse::<i32>()?; Ok(n) }

trait Speak { fn hi(&self) -> String; }
impl Speak for User { fn hi(&self) -> String { self.name.clone() } }
fn g<T: Speak>(t: &T) {}   fn h(t: &impl Speak) {}   Vec<Box<dyn Speak>>

let v = vec![1, 2, 3];  v.iter().map(|x| x * 2).filter(|x| *x > 2).collect::<Vec<_>>();
let mut m = HashMap::new();  *m.entry("k").or_insert(0) += 1;

&T (borrow)  &mut T (mutable borrow)  Box<T>  Rc<T>  Arc<T>  RefCell<T>  Mutex<T>
async fn a() {}  a().await  tokio::join!(a(), b())  tokio::spawn(async {})
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg(test)] mod tests { use super::*; #[test] fn t() { assert_eq!(1, 1); } }
```

## This project

```bash
cd notes-api && docker compose up -d db && cp .env.example .env && cargo run   # http://localhost:8810/docs
DATABASE_URL=postgres://notes:notes@localhost:5433/notes_db cargo test
cd .. && make deploy                                                            # http://api.localhost:8070/docs/
make destroy
```

## Glossary

| Word | Meaning |
|---|---|
| **Ownership** | each value has one owner; freed when the owner's scope ends |
| **Move** | ownership passes to another variable/function |
| **Borrow** | use a value via a reference (`&` read, `&mut` write) without owning it |
| **Borrow checker** | the compiler part that enforces borrowing rules |
| **Lifetime** | how long a reference is valid (`'a`) |
| **Crate** | a library or program; published on crates.io |
| **Module** | a named section of code inside a crate |
| **Trait** | shared behaviour a type promises to have |
| **Generic** | code that works for many types (`<T>`) |
| **Trait object** | `dyn Trait` — different types behind one interface, chosen at run time |
| **Macro** | code that writes code (`println!`, `vec!`, `#[derive]`) |
| **Option / Result** | "maybe a value" / "a value or an error" |
| **Panic** | an unrecoverable crash for bugs |
| **Closure** | a small unnamed function that can capture variables |
| **Iterator** | something that yields items one at a time; lazy |
| **Future** | an async task that hasn't finished yet |
| **Runtime (Tokio)** | the scheduler that runs futures |
| **Extractor** | an Axum handler argument filled from the request |
| **Migration** | a versioned database schema change |
| **JWT** | a signed token proving who you are |
| **Argon2** | a slow, salted password-hashing algorithm |
| **Multi-stage build** | Dockerfile that builds in one image and runs in a smaller one |
| **Distroless** | a minimal base image with no shell |

## Interview questions (short answers)

1. **What makes Rust memory-safe without a GC?** Ownership + borrowing rules checked at compile time; values are freed when their owner goes out of scope.
2. **Move vs copy vs clone?** Move transfers ownership (String, Vec); Copy types (numbers) are duplicated implicitly; clone is an explicit deep copy.
3. **The borrowing rule?** Many `&T` or one `&mut T` at a time, never both; references can't outlive the owner.
4. **`String` vs `&str`?** Owned, growable text vs a borrowed view of text.
5. **How does Rust handle null and errors?** `Option<T>` instead of null; `Result<T, E>` + `?` instead of exceptions; panics for bugs.
6. **What is a trait? Generics vs trait objects?** Shared behaviour. Generics are resolved at compile time (zero-cost); `dyn Trait` dispatches at run time and allows mixed collections.
7. **What are lifetimes?** Annotations describing how long references are valid, so the compiler can prevent dangling references.
8. **`Rc` vs `Arc`; `RefCell` vs `Mutex`?** Single-thread vs thread-safe shared ownership; runtime-checked mutability vs locking.
9. **What do `Send` and `Sync` mean?** Safe to move to / share between threads; the compiler uses them to prevent data races.
10. **Threads vs async?** Threads for CPU-bound work; async for many I/O-bound tasks on few threads. Never block inside async code.
11. **How does an Axum handler get its inputs?** Extractors in its arguments (`State`, `Path`, `Json`, custom ones via `FromRequestParts`).
12. **How do you prevent SQL injection?** Parameterised queries with placeholders and `.bind()`.
13. **How should passwords be stored?** Salted Argon2 (or bcrypt/scrypt) hashes, never plain or fast hashes.
14. **Why multi-stage Docker builds for Rust?** The toolchain is huge; ship only the binary on a minimal base image.
15. **Readiness vs liveness for this API?** Readiness checks the database (`/health`); liveness only the process (`/health/live`).
16. **How do you get zero-downtime deploys?** Readiness probes, `maxUnavailable: 0`, graceful SIGTERM handling with a short drain.

## Where to go next

- *The Rust Programming Language* ("the book"): [doc.rust-lang.org/book](https://doc.rust-lang.org/book/) — free, excellent.
- **Rustlings** — small exercises: `cargo install rustlings`.
- *Zero To Production In Rust* — a whole book on building a production web API.
- Extend `notes-api`: tags, search, pagination, refresh tokens, rate limiting, metrics (`/metrics` for Prometheus).
