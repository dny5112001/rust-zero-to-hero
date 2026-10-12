// Chapter 13 — async with tokio.
use std::time::{Duration, Instant};
use tokio::time::sleep;

async fn fetch(name: &str, ms: u64) -> String {
    sleep(Duration::from_millis(ms)).await; // pretend this is a network call
    format!("{name} ({ms}ms)")
}

#[tokio::main]
async fn main() {
    // One after another: total ~ 300 + 200 = 500ms
    let start = Instant::now();
    let a = fetch("users", 300).await;
    let b = fetch("orders", 200).await;
    println!("sequential: {a}, {b} in {}ms", start.elapsed().as_millis());

    // At the same time: total ~ 300ms (the slowest one)
    let start = Instant::now();
    let (a, b) = tokio::join!(fetch("users", 300), fetch("orders", 200));
    println!("concurrent: {a}, {b} in {}ms", start.elapsed().as_millis());

    // Background tasks
    let task = tokio::spawn(async { fetch("report", 100).await });
    println!("spawned task returned: {}", task.await.unwrap());
}
