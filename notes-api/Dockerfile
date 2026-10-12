# ---- Stage 1: build ---------------------------------------------------------
FROM rust:1.97-slim-trixie AS builder
WORKDIR /app

# Build dependencies first, in their own layer. They change rarely, so Docker
# reuses this (slow) step until Cargo.toml / Cargo.lock change.
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs && touch src/lib.rs \
    && cargo build --release && rm -rf src

# Now the real code.
COPY src ./src
COPY migrations ./migrations
RUN touch src/main.rs src/lib.rs && cargo build --release

# ---- Stage 2: run -----------------------------------------------------------
# A tiny image with no shell or package manager: less to attack, faster to pull.
FROM gcr.io/distroless/cc-debian13:nonroot
COPY --from=builder /app/target/release/notes-api /usr/local/bin/notes-api
ENV APP_HOST=0.0.0.0 APP_PORT=8800
EXPOSE 8800
# Numeric user id (not a name), so Kubernetes can verify "runAsNonRoot".
USER 65532:65532
ENTRYPOINT ["/usr/local/bin/notes-api"]
