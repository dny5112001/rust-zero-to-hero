# 07 · Error handling: Result and `?`

Example: [`basics/examples/07_errors.rs`](../basics/examples/07_errors.rs)

## Two kinds of errors

| Kind | Example | Rust's tool |
|---|---|---|
| **Expected** — can happen in normal use | user typed "abc" instead of a number, file missing, network down | `Result<T, E>` |
| **Bugs** — should never happen | index out of range, a "can't happen" case | `panic!` (crash) |

Rust has **no exceptions**. A function that can fail **says so in its type**,
and the caller must deal with it.

## Result

```rust
enum Result<T, E> {
    Ok(T),     // success, with the value
    Err(E),    // failure, with the error
}
```

```rust
match "42".parse::<u32>() {
    Ok(n) => println!("got {n}"),
    Err(e) => println!("not a number: {e}"),
}
```

## The `?` operator — pass the error up

Writing `match` everywhere gets long. `?` means: **"if this failed, return the
error from this function right now; otherwise give me the value."**

```rust
fn parse_age(input: &str) -> Result<u32, AgeError> {
    let age: u32 = input.trim().parse()?;     // error? return it
    if age > 130 {
        return Err(AgeError::TooOld(age));
    }
    Ok(age)
}
```

`?` only works inside a function that itself returns `Result` (or `Option`).

## Your own error type

An enum listing everything that can go wrong:

```rust
#[derive(Debug)]
enum AgeError {
    NotANumber(ParseIntError),
    TooOld(u32),
}
```

Two traits make it pleasant:
- `Display` — how it looks to humans (`{}`).
- `From<OtherError>` — lets `?` convert other errors into yours automatically.

The example writes these by hand so you see how they work. In real projects,
the **`thiserror`** crate writes them for you:

```rust
#[derive(Debug, thiserror::Error)]
enum AgeError {
    #[error("not a number: {0}")]
    NotANumber(#[from] ParseIntError),
    #[error("{0} is not a believable age")]
    TooOld(u32),
}
```

The notes API does exactly this in [`notes-api/src/error.rs`](../notes-api/src/error.rs),
and also turns each error into the right HTTP status code (chapter 16).

> **thiserror** for libraries and apps where callers need to know *which* error.
> **anyhow** (`anyhow::Result<T>`) for quick apps/scripts where you just want to report it.

## unwrap, expect, and panics

```rust
let n: u32 = "42".parse().unwrap();                 // crash if Err
let n: u32 = "42".parse().expect("config is valid"); // crash with your message
```

Use them in tests, examples, and for truly impossible cases — or at start-up
when the program can't run without something (the notes API uses `expect` if
`DATABASE_URL` is missing). **Don't** use them for user input or network calls.

## Option and Result work together

```rust
let maybe: Option<u32> = Some(5);
let res: Result<u32, &str> = maybe.ok_or("missing");    // Option -> Result
let opt: Option<u32> = "42".parse::<u32>().ok();         // Result -> Option
let value = "x".parse::<u32>().unwrap_or(0);             // default on error
```

## Try it

1. Run the example. Add an input `"-5"` — what error do you get, and why?
2. Write `fn read_number(path: &str) -> Result<i64, Box<dyn std::error::Error>>` that reads a file (`std::fs::read_to_string(path)?`) and parses it. `Box<dyn Error>` accepts any error type.
3. Convert `AgeError` to use `thiserror`.

## Remember

- Expected failures → `Result<T, E>`; bugs → panic.
- `?` returns the error early; your function must return `Result`.
- Use `thiserror` for clean error enums; avoid `unwrap` on real input.
