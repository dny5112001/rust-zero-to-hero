# 18 · Packing it into a Docker image

## The Dockerfile — two stages

[`notes-api/Dockerfile`](../notes-api/Dockerfile):

```dockerfile
# ---- Stage 1: build ----
FROM rust:1.97-slim-trixie AS builder
WORKDIR /app

# 1) Build only the dependencies, using a dummy main.rs
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs && touch src/lib.rs \
    && cargo build --release && rm -rf src

# 2) Then build the real code
COPY src ./src
COPY migrations ./migrations
RUN touch src/main.rs src/lib.rs && cargo build --release

# ---- Stage 2: run ----
FROM gcr.io/distroless/cc-debian13:nonroot
COPY --from=builder /app/target/release/notes-api /usr/local/bin/notes-api
EXPOSE 8800
# Numeric user id (not a name), so Kubernetes can verify "runAsNonRoot".
USER 65532:65532
ENTRYPOINT ["/usr/local/bin/notes-api"]
```

### Why two stages?

The Rust compiler and all build files are **~1.5 GB**. The finished program is
one small file. Stage 1 builds; stage 2 copies **only the program** into a tiny
base image. Result: **~60 MB** image.

### Why the dummy main.rs trick?

Docker caches each step. Dependencies change rarely, your code changes often.
Building dependencies in their own step means editing your code only rebuilds
**your** crate — minutes become seconds. (The `cargo-chef` tool automates this.)

### Why distroless?

`distroless` images contain only what the program needs to run — **no shell,
no package manager**. If someone breaks into the container, there are almost no
tools to use. It also means faster pulls and fewer security alerts.

The `cc` variant includes the C runtime library Rust programs link against.
Our database connection uses `rustls` (TLS written in Rust), so we don't need OpenSSL.

## Two real problems we hit (and fixed)

1. **"failed to download Swagger UI: curl command not found"** — the
   `utoipa-swagger-ui` crate downloads files during the build, and the slim image
   has no `curl`. Fix: enable its `vendored` feature so the files come bundled.
   Lesson: builds that download things are fragile; prefer vendored/offline.
2. **Comments on a Dockerfile instruction line** — `USER 65532:65532 # comment`
   is **not** a comment in a Dockerfile; it becomes part of the value. Comments
   must be on their own line.

## docker-compose for local development

[`notes-api/docker-compose.yml`](../notes-api/docker-compose.yml) runs Postgres
and the API together:

```bash
docker compose up -d db          # only Postgres (then `cargo run` for fast edits)
docker compose up --build        # both, as containers
docker compose logs -f api
docker compose down              # stop (add -v to also delete the database data)
```

- `depends_on: condition: service_healthy` waits until Postgres is really ready.
- Inside compose, the API reaches the database at host `db` (the service name).
- Ports: Postgres on **5433** and the API on **8810** on your Mac, to avoid clashing
  with other things on this machine (8800 was already in use).

## Useful commands

```bash
docker build -t notes-api:dev notes-api
docker images notes-api
docker run --rm -p 8810:8800 -e DATABASE_URL=... -e JWT_SECRET=... notes-api:dev
docker history notes-api:dev      # see each layer's size
```

## Remember

- Multi-stage: build with the big Rust image, run from a tiny one.
- Cache dependencies in their own layer.
- distroless + a numeric non-root user = small and safe.
