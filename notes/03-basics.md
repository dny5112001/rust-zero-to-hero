# 03 · The basics: variables, types, functions, control flow

Example: [`basics/examples/03_variables.rs`](../basics/examples/03_variables.rs) — `cargo run --example 03_variables`

## Variables don't change — unless you say so

```rust
let apples = 5;
apples = 6;            // ❌ error: cannot assign twice to immutable variable

let mut score = 0;     // mut = "this may change"
score += 10;           // ✅
```

Making things unchangeable by default prevents a lot of "who changed this?" bugs.

### Shadowing — reuse a name

```rust
let text = "42";
let text: i32 = text.parse().expect("not a number");   // a NEW variable, new type
```

## Types

Rust knows every variable's type at compile time. It usually **works it out**
for you; write it when it can't, or for clarity.

| Type | Examples | Notes |
|---|---|---|
| Integers | `i32`, `i64`, `u32`, `u64`, `usize` | `i` = can be negative, `u` = can't. `usize` = for sizes/indexes |
| Decimals | `f64`, `f32` | `f64` is the usual choice |
| True/false | `bool` | |
| One character | `char` | `'A'`, `'😀'` — single quotes |
| Text | `&str`, `String` | chapter 06 |
| Tuple | `(i32, String)` | a few values of different types together |
| Array | `[i32; 3]` | fixed size; for growable lists use `Vec` |

```rust
let point: (i32, i32) = (3, 4);
let (x, y) = point;            // unpack a tuple
let days = ["Mon", "Tue"];
println!("{}", days[1]);       // Tue
```

Numbers never silently change type. `let total = 5_i32 + 2.5;` is an error;
convert explicitly: `5 as f64 + 2.5`.

## Functions

```rust
fn area(width: u32, height: u32) -> u32 {
    width * height              // no semicolon = this is the return value
}
```

- Parameter types are **required**. `-> u32` is the return type.
- The **last expression without `;`** is returned. `return x;` also works for early exits.

### Expressions vs statements

Almost everything in Rust is an **expression** (it produces a value):

```rust
let label = if score > 5 { "good" } else { "keep going" };
```

Adding `;` turns an expression into a statement (no value). That's why the
last line of a function has no `;`.

## Control flow

```rust
if x > 10 {
    // ...
} else if x > 5 {
    // ...
} else {
    // ...
}

for day in days { }           // loop over items
for i in 1..=3 { }            // 1, 2, 3   (1..3 would be 1, 2)
while n > 0 { n -= 1; }
let found = loop {            // loop forever until break
    break 7;                  // break can return a value
};
```

There are no parentheses around conditions, and braces `{}` are always required.

## Comments

```rust
// a normal comment
/// a doc comment — shows up in `cargo doc` and in your editor's hover
```

## Try it

1. Write `fn is_even(n: i32) -> bool` and print it for 1 to 10 with a `for` loop.
2. Write FizzBuzz: 1 to 15; print "Fizz" for multiples of 3, "Buzz" for 5, "FizzBuzz" for both.
3. Remove `mut` from `score` in the example and read the error.

## Remember

- `let` = can't change; `let mut` = can change.
- Types are checked at compile time and often inferred.
- The last expression without `;` is the value of a block or function.
