# 11 · Modules, crates and testing

Example: [`basics/src/lib.rs`](../basics/src/lib.rs) and [`basics/tests/cart.rs`](../basics/tests/cart.rs) — run `cargo test` in `basics/`.

## Words first

| Word | Meaning |
|---|---|
| **Package** | a folder with a `Cargo.toml` |
| **Crate** | one thing the compiler builds: a **binary** (`src/main.rs`, a program) or a **library** (`src/lib.rs`, code for others to use) |
| **Module** | a named section inside a crate, for organising code |
| **Workspace** | several packages that build together |

A package can have both: the notes API has `src/lib.rs` (all the logic) **and**
`src/main.rs` (just starts the server). Tests use the library directly.

## Modules: folders for your code

```rust
pub mod shop {                    // a module
    pub struct Cart {
        items: Vec<(String, u32)>, // private field
    }

    impl Cart {
        pub fn new() -> Self { Cart { items: Vec::new() } }   // pub = usable from outside
    }
}

use basics::shop::Cart;           // bring it into scope
```

**Everything is private by default.** Add `pub` to share it. This lets you
change the inside of a module without breaking the outside.

In bigger projects, each module is a file. The notes API:

```
src/
├── main.rs            # starts the server
├── lib.rs             # pub mod app; pub mod auth; ...
├── app.rs             # routes
├── error.rs
├── auth/
│   ├── mod.rs         # the `auth` module itself + `pub mod jwt; pub mod password;`
│   ├── jwt.rs
│   └── password.rs
└── routes/ ...
```

`mod x;` in a file means "load module `x` from `x.rs` or `x/mod.rs`". Paths:
`crate::` = from the top of this crate, `super::` = the parent module, `self::` = this module.

## Using libraries (crates)

```bash
cargo add serde --features derive
cargo add tokio --features full
```

Each `features` switch turns on optional parts of a crate. Find crates on
[crates.io](https://crates.io), read docs on [docs.rs](https://docs.rs).

Popular crates you'll meet: `serde` (JSON), `tokio` (async), `axum` (web),
`sqlx` (databases), `reqwest` (HTTP client), `clap` (command-line args),
`tracing` (logs), `thiserror`/`anyhow` (errors), `chrono` (dates), `uuid`.

## Testing

### Unit tests — next to the code

```rust
#[cfg(test)]               // only compiled for `cargo test`
mod tests {
    use super::*;          // see everything in the parent module, even private things

    #[test]
    fn big_orders_get_ten_percent_off() {
        let mut cart = Cart::new();
        cart.add("phone", 200_000);
        assert_eq!(cart.total(), 180_000);
    }
}
```

### Integration tests — from the outside

Files in `tests/` use your library like a real user, through `pub` items only:

```rust
// tests/cart.rs
use basics::shop::Cart;

#[test]
fn checkout_flow() { /* ... */ }
```

### Asserts

```rust
assert!(condition);
assert_eq!(left, right);        // shows both values when it fails
assert_ne!(left, right);
#[should_panic]                 // the test passes only if the code panics
```

### Running

```bash
cargo test                      # everything
cargo test discount             # only tests with "discount" in the name
cargo test -- --nocapture       # show println! output
```

The notes API has 5 unit tests (password hashing, JWT) and 7 integration tests
that run against a **real Postgres** (chapter 17).

### Doc tests

Code in `///` comments is run as a test too, so your examples can't go stale.

## Try it

1. In `basics/`, run `cargo test`. Break the discount rule and watch a test fail.
2. Add a `remove(&mut self, name: &str)` method to `Cart` with a test.
3. Split `shop` into its own file `src/shop.rs` (put `pub mod shop;` in `lib.rs`).

## Remember

- Package → crates (lib/bin) → modules. Private by default; `pub` to share.
- `cargo add` for libraries; features turn on optional parts.
- Unit tests in `#[cfg(test)] mod tests`; integration tests in `tests/`.
