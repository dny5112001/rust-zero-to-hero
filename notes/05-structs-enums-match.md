# 05 · Structs, enums, match and Option

Example: [`basics/examples/05_structs_enums.rs`](../basics/examples/05_structs_enums.rs)

## Structs — your own data types

A **struct** groups related values under one name, like a form with fields:

```rust
#[derive(Debug)]          // lets you print it with {:?}
struct User {
    name: String,
    age: u32,
}

let asha = User { name: String::from("Asha"), age: 17 };
println!("{} is {}", asha.name, asha.age);
```

### Methods with `impl`

```rust
impl User {
    // "associated function" (no self) — like a constructor
    fn new(name: &str, age: u32) -> Self {
        User { name: name.to_string(), age }
    }

    fn is_adult(&self) -> bool {        // &self = read the struct
        self.age >= 18
    }

    fn birthday(&mut self) {            // &mut self = change the struct
        self.age += 1;
    }
}

let mut asha = User::new("Asha", 17);   // call with ::
asha.birthday();                         // call methods with .
```

`self`, `&self`, `&mut self` follow the ownership rules from chapter 04.

## Enums — one of several choices

An **enum** is a value that is exactly **one** of a fixed set of options. Each
option can carry its own data:

```rust
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Dot,
}
```

## match — handle every case

```rust
fn area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
        Shape::Rectangle { width, height } => width * height,
        Shape::Dot => 0.0,
    }
}
```

`match` must cover **every** option. Add a new shape and forget to handle it?
**Compile error.** This "exhaustiveness check" catches lots of bugs.

`match` works on numbers and text too:

```rust
match code {
    200 => "ok",
    404 => "not found",
    500..=599 => "server error",
    _ => "something else",          // _ = anything else
}
```

## Option — Rust has no null

Many languages use `null` for "no value", and forgetting to check it is the
famous "billion-dollar mistake". Rust uses an enum instead:

```rust
enum Option<T> {
    Some(T),     // there is a value
    None,        // there isn't
}
```

```rust
fn find_user<'a>(users: &'a [User], name: &str) -> Option<&'a User> {
    users.iter().find(|u| u.name == name)
}

match find_user(&users, "Ravi") {
    Some(user) => println!("found {}", user.name),
    None => println!("not found"),
}
```

You **can't** use an `Option<User>` as a `User` by accident — the compiler
makes you deal with `None`.

### Handy shortcuts

```rust
if let Some(user) = find_user(&users, "Ravi") { /* only runs when found */ }

let age = find_user(&users, "Asha").map(|u| u.age).unwrap_or(0);  // default if None
let user = maybe_user.expect("user must exist");                  // crash with a message if None
```

`unwrap()` / `expect()` crash on `None`. Fine in examples and tests; in real code
prefer `match`, `if let`, `unwrap_or`, or `?` (chapter 07).

## Try it

1. Add a `Triangle { base: f64, height: f64 }` to `Shape` — see the compiler point at the `match` you must update.
2. Give `User` an `email: Option<String>` field and print "no email" when it's `None`.
3. Write an enum `TrafficLight { Red, Yellow, Green }` with a method `next(&self) -> TrafficLight`.

## Remember

- `struct` = fields together; `impl` = methods (`&self`, `&mut self`).
- `enum` = one of several options, each with its own data.
- `match` must handle every case. `Option` replaces null: `Some(x)` or `None`.
