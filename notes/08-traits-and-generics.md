# 08 · Traits and generics — sharing behaviour

Example: [`basics/examples/08_traits_generics.rs`](../basics/examples/08_traits_generics.rs)

## Traits — "can do" contracts

A **trait** lists abilities a type promises to have. Like a job description:
"anyone who is a `Speak`er must be able to give their `name`".

```rust
trait Speak {
    fn name(&self) -> String;                 // must be written by each type

    fn greet(&self) -> String {               // a default — types get it for free
        format!("Hi, I'm {}", self.name())
    }
}

struct Robot { id: u32 }

impl Speak for Robot {
    fn name(&self) -> String {
        format!("RX-{}", self.id)
    }
}
```

Other languages call these **interfaces**.

## Traits you'll use every day

| Trait | Gives you | Usually get it with |
|---|---|---|
| `Debug` | print with `{:?}` | `#[derive(Debug)]` |
| `Clone` | `.clone()` | `#[derive(Clone)]` |
| `Copy` | copied instead of moved (small simple types) | `#[derive(Clone, Copy)]` |
| `PartialEq` | `==` | `#[derive(PartialEq)]` |
| `PartialOrd`, `Ord` | `<`, sorting | derive |
| `Default` | `Type::default()` | derive or write |
| `Display` | print with `{}` | write it yourself |
| `From` / `Into` | convert one type into another | write it yourself |
| `Serialize`, `Deserialize` | to/from JSON (serde crate) | `#[derive(Serialize)]` |

`#[derive(...)]` writes the code for you. In the notes API, `#[derive(Serialize, sqlx::FromRow)]`
means "turn this into JSON" and "fill this from a database row".

## Generics — one function, many types

Instead of writing `largest_i32`, `largest_f64`, `largest_char`...:

```rust
fn largest<T: PartialOrd + Copy>(items: &[T]) -> T {
    let mut best = items[0];
    for &item in items {
        if item > best { best = item; }
    }
    best
}

largest(&[3, 9, 2]);        // 9
largest(&['x', 'b']);       // 'x'
```

`T` is a placeholder type. `T: PartialOrd + Copy` is a **trait bound**: "any
type, as long as it can be compared and copied". The compiler makes a
specialised copy for each type you use, so generics cost **nothing** at run time.

Longer bounds read better with `where`:

```rust
fn report<T>(item: &T) -> String
where
    T: Speak + std::fmt::Debug,
{
    format!("{item:?} says {}", item.greet())
}
```

You already use generic types: `Vec<T>`, `Option<T>`, `Result<T, E>`, `HashMap<K, V>`.

## `impl Trait` — the short form

```rust
fn introduce(thing: &impl Speak) {        // "anything that implements Speak"
    println!("{}", thing.greet());
}

fn make_iter() -> impl Iterator<Item = u32> {   // "some iterator; you don't need the exact type"
    (1..=3).map(|n| n * 10)
}
```

## Trait objects — a list of different types

Generics need one type per use. To keep **different** types in one list, use
`dyn Trait` behind a pointer:

```rust
let crowd: Vec<Box<dyn Speak>> = vec![Box::new(Dog), Box::new(Robot { id: 1 })];
for member in &crowd {
    println!("{}", member.greet());
}
```

The right method is picked **at run time** (tiny cost). Use generics by default;
trait objects when you need a mixed collection or a plugin-style design.

## In the notes API

[`notes-api/src/auth/mod.rs`](../notes-api/src/auth/mod.rs) implements Axum's
`FromRequestParts` trait for `AuthUser`. That's how simply adding
`user: AuthUser` to a handler makes it require a login — Axum calls the trait
method before your handler runs.

## Try it

1. Add a `Cat` that implements `Speak` and add it to the crowd.
2. Implement `Display` for `Point` so `println!("{p}")` prints `(1, 2)`.
3. Write a generic `fn print_all<T: std::fmt::Debug>(items: &[T])`.

## Remember

- Trait = shared behaviour ("can do"). `#[derive]` gets common ones for free.
- Generics + trait bounds = one function for many types, with no run-time cost.
- `dyn Trait` = different types in one collection, decided at run time.
