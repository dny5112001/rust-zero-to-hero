# 04 · Ownership and borrowing — Rust's big idea

**This is the chapter that makes Rust, Rust.** Take your time with it.

Example: [`basics/examples/04_ownership.rs`](../basics/examples/04_ownership.rs)

## The three ownership rules

1. Every value has **one owner** (a variable).
2. There is only **one owner at a time**.
3. When the owner goes **out of scope** (its `{ }` block ends), the value is **freed**.

That's how Rust frees memory without a garbage collector: the compiler knows
exactly when each owner ends, and inserts the clean-up there.

```rust
{
    let s = String::from("hello");   // s owns the text
    // use s...
}                                    // s ends here → the memory is freed
```

## Moving: handing over ownership

```rust
let a = String::from("hello");
let b = a;              // ownership MOVES from a to b
println!("{a}");        // ❌ error: borrow of moved value: `a`
```

Like handing someone your **only key**: after you give it away, you can't open
the door. Rust does this so the text is never freed twice.

Passing a value to a function also moves it:

```rust
fn takes_ownership(s: String) { /* s is freed when this ends */ }

let c = String::from("hi");
takes_ownership(c);
// c can't be used any more
```

### Copy types don't move

Small, simple values — numbers, `bool`, `char`, and tuples of those — are just
**copied**:

```rust
let x = 5;
let y = x;      // a copy
println!("{x} {y}");   // ✅ both fine
```

### clone — make a real copy

```rust
let c = b.clone();     // a second, independent String
```

`clone` costs time and memory. It's fine to use while learning; later you'll
mostly **borrow** instead.

## Borrowing: lending without giving away

A **reference** (`&`) lets someone **use** a value without owning it — like
lending a book: they read it, then you still have it.

```rust
fn print_len(s: &str) {
    println!("{} bytes", s.len());
}

let word = String::from("borrowed");
print_len(&word);          // lend it
println!("{word}");        // ✅ still mine
```

### Mutable borrowing

To let someone **change** it, lend it mutably with `&mut`:

```rust
fn add_item(list: &mut Vec<i32>) {
    list.push(4);
}

let mut list = vec![1, 2, 3];
add_item(&mut list);
```

## The borrowing rule (the "borrow checker")

At any moment you can have **either**:

- **any number of readers** (`&T`), **or**
- **exactly one writer** (`&mut T`)

— **never both at once.**

Like a shared document: many people can read it at the same time, or one person
can edit it — but nobody reads while someone is halfway through editing.

```rust
let mut v = vec![1, 2, 3];
let first = &v[0];       // a reader
v.push(4);               // ❌ a writer while a reader is still in use
println!("{first}");
```

Why is that an error? `push` might need to move the whole list to a bigger spot
in memory — and then `first` would point at freed memory. In C++ this compiles
and crashes later. **Rust refuses to compile it.** This one rule is what
prevents use-after-free bugs and data races between threads.

## References must not outlive the owner

```rust
fn dangling() -> &String {
    let s = String::from("hi");
    &s                       // ❌ s is freed at the end, so the reference would point to nothing
}
```

Fix: return the `String` itself (move ownership out). Chapter 10 covers
**lifetimes**, the notation for "how long does this reference live?".

## Slices: borrowing part of something

```rust
let sentence = String::from("hello world");
let first: &str = &sentence[0..5];      // "hello" — a view into the String, no copy
```

`&str` is a **string slice**. Functions that only *read* text should take
`&str` — then they accept both `String` (via `&my_string`) and literals like `"hi"`.

## How to "fight the borrow checker" (and win)

When you get a borrow error:
1. **Read the whole message** — it shows where the borrow starts and where it conflicts.
2. Ask: **does this function need to own it?** If it only reads, take `&T`.
3. **Shorten borrows**: finish using a reference before changing the value.
4. If you truly need two owners, see `Rc` / `Arc` in chapter 10.
5. When stuck while learning, `.clone()` is an acceptable escape hatch.

## Try it

1. In the example, uncomment `println!("{a}")` and read the error.
2. Write `fn shout(s: &mut String)` that adds `"!"` to the end. Call it twice.
3. Write `fn longest_word(text: &str) -> &str`.

## Remember

- One owner; when it goes out of scope, the value is freed.
- Assigning or passing a `String`/`Vec` **moves** it; numbers are copied.
- Borrow with `&` (read) or `&mut` (write). Many readers **or** one writer, never both.
