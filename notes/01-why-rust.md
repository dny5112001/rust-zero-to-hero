# 01 · Why Rust?

## The problem Rust solves

Programming languages usually make you pick one:

| Fast, but risky | Safe, but slower |
|---|---|
| **C, C++** — you manage memory yourself. Forget to free it → leaks. Free it twice or use it after freeing → crashes and security holes. | **Python, JavaScript, Java, Go** — a **garbage collector** cleans memory for you, but it uses extra memory and pauses your program now and then. |

**Rust gives you both**: as fast as C/C++, and memory-safe — **without** a garbage
collector. It does this with a set of rules called **ownership** (chapter 04),
which the **compiler checks before your program ever runs**.

## What that means in practice

- **Whole types of bugs can't happen**: no null-pointer crashes (Rust has no null), no use-after-free, no data races between threads.
- **Fast and small**: our notes API's Docker image is ~60 MB and runs in ~25 MB of RAM.
- **"If it compiles, it usually works."** The compiler is strict, but its error messages are famously helpful — it often tells you the exact fix.
- **Great tools built in**: `cargo` builds, tests, formats, lints and manages libraries.

## The honest downside

Rust is **harder to start** than Python or Go. You'll "fight the borrow checker"
for a couple of weeks. That's normal — it's teaching you to think about who owns
data. After that, it gets much easier, and the habits make you better in every language.

## Where Rust is used

- Web backends and APIs (Cloudflare, Discord, AWS — Firecracker, the engine behind Lambda, is Rust)
- Command-line tools (ripgrep, fd, bat, uv, ruff)
- Infrastructure (Linkerd proxy, parts of Docker/Kubernetes tooling)
- Browsers (Firefox), operating systems (parts of Linux, Windows, Android)
- WebAssembly, embedded devices, games

## How this course works

```
Part 1 (ch 02–10)  The language        → small programs in basics/examples/
Part 2 (ch 11–13)  Bigger programs     → modules, tests, threads, async
Part 3 (ch 14–17)  A real web API      → notes-api/ (Axum + Postgres + JWT)
Part 4 (ch 18–20)  Shipping it         → Docker → Kubernetes → Terraform
```

## Remember

- Rust = speed of C/C++ + safety, with no garbage collector.
- The compiler checks ownership rules before your code runs.
- It's harder at first; the compiler's error messages are your teacher.
