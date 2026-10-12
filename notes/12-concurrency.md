# 12 · Doing many things at once: threads

Example: [`basics/examples/12_threads.rs`](../basics/examples/12_threads.rs)

Rust's slogan here is **"fearless concurrency"**: the same ownership rules from
chapter 04 stop the classic multi-threading bugs **at compile time**.

## Threads

A **thread** is a separate line of work that runs at the same time as others
(on another CPU core if available).

```rust
use std::thread;

let handle = thread::spawn(|| {
    "work done"
});
let result = handle.join().unwrap();   // wait for it and get its result
```

To use outside data in a thread, `move` it in (the thread might outlive the
current function):

```rust
let id = 7;
thread::spawn(move || println!("thread {id}"));
```

## Option 1: send messages (channels)

"Don't share memory; share messages." Threads send values down a **channel**,
like passing notes:

```rust
use std::sync::mpsc;

let (tx, rx) = mpsc::channel();      // tx = sender, rx = receiver
for id in 0..3 {
    let tx = tx.clone();
    thread::spawn(move || tx.send(id * 10).unwrap());
}
drop(tx);                            // close ours, so the loop ends when all senders are done
for value in rx { println!("{value}"); }
```

Sending a value **moves** it — the sender can't touch it afterwards, so two
threads can never change it at once.

## Option 2: share data with Arc + Mutex

```rust
use std::sync::{Arc, Mutex};

let counter = Arc::new(Mutex::new(0));
for _ in 0..8 {
    let counter = Arc::clone(&counter);
    thread::spawn(move || {
        *counter.lock().unwrap() += 1;      // lock, change, unlock automatically
    });
}
```

- `Arc` = several threads own it (chapter 10).
- `Mutex` = **one at a time**: `lock()` waits until no-one else holds it. Like a
  bathroom key — one person at a time.

The example adds 1 eight thousand times from 8 threads and **always** gets 8000.

## What the compiler stops you from doing

```rust
let mut count = 0;
thread::spawn(|| count += 1);     // ❌ closure may outlive `count`; also two threads could change it
```

```rust
let shared = Rc::new(5);
thread::spawn(move || println!("{shared}"));  // ❌ `Rc` cannot be sent between threads safely
```

Two marker traits make this work: `Send` (safe to move to another thread) and
`Sync` (safe to share between threads). The compiler adds them automatically
when it's safe — and refuses when it isn't. In other languages these bugs show
up randomly in production; in Rust they're compile errors.

## Easy parallelism: rayon

For "do this to every item, using all cores", the `rayon` crate changes
`.iter()` to `.par_iter()` and you're done.

## Threads vs async

| | Threads (this chapter) | Async (next chapter) |
|---|---|---|
| Good for | heavy **calculation** (CPU-bound) | lots of **waiting** — network, database, files (I/O-bound) |
| Cost each | ~MBs of memory | ~KBs |
| Thousands at once? | not really | yes, easily |

A web server mostly **waits** on the network and database, so it uses async.

## Remember

- `thread::spawn(move || ...)`, then `join()` to wait.
- Share by messaging (channels) or by `Arc<Mutex<T>>`.
- `Send`/`Sync` let the compiler reject data races before your code runs.
