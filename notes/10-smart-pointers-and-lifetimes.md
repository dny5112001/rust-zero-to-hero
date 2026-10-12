# 10 · Smart pointers and lifetimes

Example: [`basics/examples/10_smart_pointers.rs`](../basics/examples/10_smart_pointers.rs)

Sometimes the "one owner" rule is too strict. **Smart pointers** are types that
bend it safely.

## Box — put something on the heap

```rust
let big = Box::new([0u8; 1_000_000]);
```

Normal values live on the **stack** (fast, but size must be known at compile time).
`Box` puts them on the **heap** and keeps a small pointer on the stack.

You need it for **recursive types**, whose size would otherwise be infinite:

```rust
enum List {
    Item(i32, Box<List>),   // a list contains a (boxed) smaller list
    End,
}
```

And for trait objects: `Box<dyn Speak>` (chapter 08).

## Rc — several owners (one thread)

`Rc` = "reference counted". Several parts of your program **share** one value;
it's freed when the **last** owner goes away.

```rust
use std::rc::Rc;

let config = Rc::new(String::from("shared config"));
let a = Rc::clone(&config);       // cheap: just adds 1 to a counter
let b = Rc::clone(&config);
Rc::strong_count(&config);        // 3
```

Like a shared flat: everyone has a key, and the lights go off when the last
person leaves. `Rc` values are **read-only**.

## RefCell — change something that's shared

`RefCell` moves the borrow rule check from compile time to **run time**:

```rust
use std::cell::RefCell;

let cart = Rc::new(RefCell::new(Vec::new()));
cart.borrow_mut().push("book");        // a writer
println!("{:?}", cart.borrow());       // a reader
```

Break the rule (two `borrow_mut` at once) → **panic at run time**. Use it
sparingly; it's a sign to double-check your design.

## Arc and Mutex — the thread-safe versions

| One thread | Many threads |
|---|---|
| `Rc<T>` | `Arc<T>` (atomic reference count) |
| `RefCell<T>` | `Mutex<T>` or `RwLock<T>` |

`Arc<Mutex<T>>` is the classic "shared, changeable data between threads" (chapter 12).
The compiler **won't let** you send an `Rc` to another thread — that's how
Rust prevents data races.

In the notes API, `AppState` is cloned for every request. Cheap, because the
database pool (`PgPool`) is internally an `Arc`.

## Lifetimes — how long does a reference live?

Every reference has a **lifetime**: the stretch of code where it's valid. Usually
the compiler works it out. Sometimes it needs a hint:

```rust
fn longer(a: &str, b: &str) -> &str {      // ❌ which input does the result borrow from?
    if a.len() >= b.len() { a } else { b }
}
```

```rust
fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {   // ✅
    if a.len() >= b.len() { a } else { b }
}
```

`'a` (say "tick a") is a **name for a lifetime**. This reads: "the result lives
as long as **both** inputs are alive". It doesn't change how long anything
lives — it just **describes** the relationship so the compiler can check callers.

You'll mostly see lifetimes:
- on functions returning a reference derived from inputs (above),
- on structs that hold references: `struct Parser<'a> { text: &'a str }`,
- as `'static` = lives for the whole program (e.g. string literals: `&'static str`).

**Beginner tip:** if lifetimes get painful, store **owned** data (`String`
instead of `&str`) in your structs. It's simpler and usually fast enough.

## Try it

1. Add a `fn len(list: &List) -> usize` for the boxed list.
2. Make two `Rc` clones of a `Vec`, then drop one and print `strong_count`.
3. Try to `borrow_mut()` a `RefCell` twice at the same time and read the panic message.

## Remember

- `Box` = heap value with one owner; needed for recursive types and `dyn Trait`.
- `Rc`/`Arc` = shared owners; `RefCell`/`Mutex` = change shared data safely.
- Lifetimes (`'a`) describe how long references are valid; owned data avoids most of them.
