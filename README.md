# Rust: Zero to Hero — and shipping it

Plain-English notes for learning Rust from nothing, then building a real web
API and deploying it with **Docker**, **Kubernetes** and **Terraform** — all on
your laptop, for free.

```
basics/      small runnable programs for the language chapters   (cargo run --example 04_ownership)
notes-api/   the web API: Axum + Postgres + JWT login             (cargo run / docker compose up)
deploy/      Terraform: kind cluster → Traefik → Postgres + app   (make deploy)
notes/       the chapters
```

## Chapters

### Part 1 — The language
| # | Chapter | You'll learn |
|---|---|---|
| 01 | [Why Rust?](notes/01-why-rust.md) | Fast *and* safe — and the honest downside |
| 02 | [Install Rust and meet Cargo](notes/02-install-and-cargo.md) | rustup, cargo, your first program |
| 03 | [The basics](notes/03-basics.md) | Variables, types, functions, if/loops |
| 04 | [Ownership and borrowing](notes/04-ownership-and-borrowing.md) | **Rust's big idea** — take your time |
| 05 | [Structs, enums, match, Option](notes/05-structs-enums-match.md) | Your own types; no more null |
| 06 | [Collections and strings](notes/06-collections-and-strings.md) | Vec, String vs &str, HashMap |
| 07 | [Error handling](notes/07-error-handling.md) | Result, `?`, thiserror |
| 08 | [Traits and generics](notes/08-traits-and-generics.md) | Shared behaviour, one function for many types |
| 09 | [Closures and iterators](notes/09-closures-and-iterators.md) | map/filter/collect, lazy and fast |
| 10 | [Smart pointers and lifetimes](notes/10-smart-pointers-and-lifetimes.md) | Box, Rc, RefCell, Arc, `'a` |

### Part 2 — Bigger programs
| # | Chapter | You'll learn |
|---|---|---|
| 11 | [Modules, crates and testing](notes/11-modules-crates-testing.md) | Organising code, `cargo test` |
| 12 | [Threads](notes/12-concurrency.md) | Channels, Arc<Mutex>, fearless concurrency |
| 13 | [Async and Tokio](notes/13-async-and-tokio.md) | async/await, join!, don't block |

### Part 3 — A real web API ([`notes-api/`](notes-api))
| # | Chapter | You'll learn |
|---|---|---|
| 14 | [Building an API with Axum](notes/14-web-api-with-axum.md) | Routes, extractors, JSON, validation, docs |
| 15 | [Postgres with SQLx](notes/15-database-with-sqlx.md) | Pools, migrations, safe queries |
| 16 | [Login, tokens and errors](notes/16-auth-and-errors.md) | Argon2, JWT, one error type |
| 17 | [Testing the API](notes/17-testing-the-api.md) | A fresh database per test |

### Part 4 — Shipping it
| # | Chapter | You'll learn |
|---|---|---|
| 18 | [Docker](notes/18-docker.md) | Multi-stage builds, a 60 MB image |
| 19 | [Kubernetes](notes/19-kubernetes.md) | Probes, graceful shutdown, zero-downtime rollouts |
| 20 | [Terraform](notes/20-terraform-deploy.md) | The whole platform from code; the path to AWS |
| 21 | [Cheat sheet, glossary, interview](notes/21-cheat-sheet-and-glossary.md) | Check yourself |

## Quick start

You need: Rust (`rustup`), Docker Desktop, and for Part 4 `kind`, `kubectl`, `helm`, `terraform`.

```bash
# Part 1–2: language examples
cd basics && cargo run --example 04_ownership && cargo test

# Part 3: the API locally
cd notes-api
cp .env.example .env
docker compose up -d db
cargo run                                   # http://localhost:8810/docs
DATABASE_URL=postgres://notes:notes@localhost:5433/notes_db cargo test

# Part 4: deploy to a local Kubernetes cluster with Terraform
cd .. && make deploy                        # http://api.localhost:8070/docs/
make destroy
```

Ports on your Mac: **5433** (Postgres), **8810** (API via `cargo run`/compose), **8070** (Kubernetes ingress).

## About notes-api

It started as your own skeleton (Axum, SQLx, config module, `/health`), and was
finished here with: users and notes tables (migrations), sign-up/login with
Argon2 + JWT, per-user notes CRUD, input validation, one error type, Swagger docs,
readiness/liveness endpoints, graceful shutdown, and 12 tests. The original
folder `~/Desktop/Projects/Personal/rust/notes-api` was not changed.

## Related notes

- [terraform-zero-to-hero](https://github.com/dny5112001/terraform-zero-to-hero)
- [kubernetes-zero-to-hero](https://github.com/dny5112001/kubernetes-zero-to-hero)
- [k8s-terraform-lab](https://github.com/dny5112001/k8s-terraform-lab)
