# 06 · Collections and strings

Example: [`basics/examples/06_collections.rs`](../basics/examples/06_collections.rs)

## Vec — a growable list

```rust
let mut scores = vec![70, 85, 92];     // vec! = quick way to make one
scores.push(60);
let first = scores[0];                 // crashes if out of range
let maybe = scores.get(10);            // Option: None if out of range — safer
scores.sort();
scores.len();
scores.contains(&85);
scores.remove(0);

for s in &scores { }                   // borrow each item
for s in &mut scores { *s += 5; }      // change each item (* = "the value behind the reference")
```

A `Vec` holds items of **one type**. Mixing types needs an enum or trait objects (chapter 08).

## Strings: `String` vs `&str`

The most confusing part for beginners — but it's just ownership again:

| | `String` | `&str` |
|---|---|---|
| What | text you **own**, can grow | a **borrowed view** of some text |
| Like | your own notebook | a photo of a page |
| Make one | `String::from("hi")`, `"hi".to_string()`, `format!(...)` | `"hi"` (literals), `&my_string` |
| Use in function parameters | when you need to keep it | when you only read it ✅ (most common) |

```rust
let mut greeting = String::from("Hello");
greeting.push_str(", world");
let s = format!("{greeting}! You are {} years old", 30);   // build strings like println!

"  hi  ".trim();
"a,b,c".split(',');
"Hello".to_uppercase();
"hello world".contains("world");
"hello".replace('l', "L");
```

### Text is UTF-8

Rust strings support every language and emoji. That's why you **can't** do
`s[0]` — a "character" may take 1 to 4 bytes. Use:

```rust
"नमस्ते".chars().count();          // characters
"नमस्ते".len();                    // bytes (larger!)
"hello".chars().next();          // first character, as Option<char>
```

## HashMap — look things up by key

```rust
use std::collections::HashMap;

let mut stock: HashMap<String, u32> = HashMap::new();
stock.insert("apples".to_string(), 5);
stock.get("apples");                             // Option<&u32>
*stock.entry("apples".to_string()).or_insert(0) += 3;   // add to existing or start at 0
stock.remove("apples");

for (fruit, count) in &stock {
    println!("{fruit}: {count}");
}
```

The `entry(...).or_insert(...)` pattern is the classic way to **count** things:

```rust
let mut counts: HashMap<&str, usize> = HashMap::new();
for word in text.split_whitespace() {
    *counts.entry(word).or_insert(0) += 1;
}
```

HashMaps have **no order**. Need sorted keys? Use `BTreeMap`.

## Other collections

| Type | Use for |
|---|---|
| `HashSet` | unique items, fast "is it in there?" |
| `BTreeMap` / `BTreeSet` | like HashMap/HashSet but sorted |
| `VecDeque` | a queue (add/remove at both ends) |

## Try it

1. Read a sentence and print each word with how many times it appears, most common first.
2. Store a phone book (`HashMap<String, String>`) and look up a name that doesn't exist — handle the `None`.
3. Reverse a string by characters: `"hello".chars().rev().collect::<String>()`.

## Remember

- `Vec<T>` for lists, `HashMap<K, V>` for lookups.
- `String` = owned text, `&str` = borrowed text. Take `&str` in functions that only read.
- Use `.get()` when an item might be missing — it returns an `Option`.
