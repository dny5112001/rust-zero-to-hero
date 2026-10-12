# 02 · Install Rust and meet Cargo

## Install

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustc --version
cargo --version
```

- **rustup** — installs and updates Rust (`rustup update`).
- **rustc** — the compiler. You rarely call it directly.
- **cargo** — your everyday tool: build, run, test, add libraries.

In VS Code, install **rust-analyzer** — it shows types, errors and fixes as you type.

## A new project

```bash
cargo new hello
cd hello
cargo run
```

```
hello/
├── Cargo.toml     # the project's name, version and dependencies ("crates")
└── src/
    └── main.rs    # your code
```

`src/main.rs`:

```rust
fn main() {
    println!("Hello, world!");
}
```

- `fn main()` — where every program starts.
- `println!` — prints a line. The `!` means it's a **macro** (code that writes code), not a normal function.

Printing values:

```rust
let name = "Asha";
println!("Hello, {name}! 2 + 3 = {}", 2 + 3);   // {} = put a value here
println!("{:?}", vec![1, 2, 3]);                // {:?} = debug view of anything
println!("{:.2}", 3.14159);                     // 2 decimal places
```

## Cargo commands

```bash
cargo run              # build + run
cargo build            # build (debug: fast to compile, slower to run)
cargo build --release  # optimised build for production (target/release/)
cargo check            # just check for errors — fastest feedback while coding
cargo test             # run tests
cargo fmt              # format code the standard way
cargo clippy           # extra hints: bugs, simpler ways to write things
cargo add serde        # add a library (crate) to Cargo.toml
cargo doc --open       # documentation for your code and all its libraries
```

Libraries are called **crates** and live on [crates.io](https://crates.io).
`Cargo.lock` records the exact versions used — commit it for programs.

## The examples in this repo

All language examples are in [`basics/examples/`](../basics/examples). Run one with:

```bash
cd basics
cargo run --example 02_hello
```

## Try it

1. `cargo new playground`, then print your name and age.
2. Make a typo on purpose (remove a `;` or a `"`) and read the compiler's message. Notice how it points to the exact spot and often suggests the fix.

## Remember

- `cargo new`, `cargo run`, `cargo check`, `cargo test`, `cargo fmt`, `cargo clippy`.
- Libraries = crates; add them with `cargo add`.
- Read compiler errors slowly — they're usually right.
