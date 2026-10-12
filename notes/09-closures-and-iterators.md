# 09 · Closures and iterators

Example: [`basics/examples/09_closures_iterators.rs`](../basics/examples/09_closures_iterators.rs)

## Closures — small unnamed functions

```rust
let tax = 0.18;
let with_tax = |price: f64| price * (1.0 + tax);   // uses `tax` from around it
with_tax(100.0);                                   // 118.0
```

`|args| body` — like a mini function you can store in a variable or pass to
another function. Unlike normal functions, closures can **capture** variables
from where they're written.

If a closure changes something it captured, it must be `mut`:

```rust
let mut counter = 0;
let mut tick = || counter += 1;
tick();
```

`move` makes the closure **take ownership** of what it captures. Needed when
the closure outlives the current function — for example when sent to another
thread (chapter 12):

```rust
let name = String::from("worker");
std::thread::spawn(move || println!("{name}"));
```

## Iterators — process lists step by step

An **iterator** gives you items one at a time. You chain small steps, each
describing *what* you want:

```rust
let prices = [120.0, 45.5, 300.0, 9.99];

let expensive: Vec<f64> = prices.iter()        // go through each item (borrowed)
    .copied()                                  // &f64 -> f64
    .filter(|p| *p > 50.0)                     // keep only some
    .collect();                                // gather into a Vec

let total: f64 = prices.iter().sum();
```

Read it like a sentence: "take the prices, keep the ones over 50, collect them".

### Lazy: nothing happens until you ask

`map` and `filter` don't do anything by themselves. Work only happens when a
**consumer** pulls the items: `collect`, `sum`, `count`, `for`, `find`, `any`...
This is why chains are fast — Rust fuses them into one tight loop, as fast as
writing the loop by hand.

### The most useful methods

| Method | Does |
|---|---|
| `map(|x| ...)` | change each item |
| `filter(|x| ...)` | keep items where it's true |
| `enumerate()` | add the position: `(0, item)`, `(1, item)`... |
| `zip(other)` | pair items with another iterator |
| `take(n)`, `skip(n)` | first n / all but first n |
| `rev()` | backwards |
| `flat_map`, `flatten` | lists of lists → one list |
| `collect()` | into `Vec`, `String`, `HashMap`... |
| `sum()`, `count()`, `min()`, `max()` | totals |
| `find(|x| ...)` | first match → `Option` |
| `any`, `all` | true/false questions |
| `fold(start, |acc, x| ...)` | build up one result |

```rust
let answer: u32 = (1..=10).filter(|n| n % 2 == 0).map(|n| n * n).sum();   // 220
```

### iter, iter_mut, into_iter

| Call | Gives | The original |
|---|---|---|
| `v.iter()` | `&T` (read) | still yours |
| `v.iter_mut()` | `&mut T` (change in place) | still yours |
| `v.into_iter()` | `T` (owned) | used up (moved) |

`for x in &v` = `v.iter()`; `for x in v` = `v.into_iter()`.

## Try it

1. From `["asha", "ravi", "meera"]`, build a `Vec<String>` of names longer than 4 letters, capitalised.
2. Use `fold` to find the longest word in a sentence.
3. Turn `vec![("a", 1), ("b", 2)]` into a `HashMap` with `.into_iter().collect()`.

## Remember

- Closures = small functions that can capture variables; `move` to take ownership.
- Iterator chains are lazy and as fast as hand-written loops.
- `iter()` borrows, `iter_mut()` changes in place, `into_iter()` consumes.
