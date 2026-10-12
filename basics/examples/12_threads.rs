// Chapter 12 — threads, channels, Arc<Mutex>.
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

fn main() {
    // Spawn threads and wait for them.
    let handles: Vec<_> = (1..=3)
        .map(|id| thread::spawn(move || format!("thread {id} done")))
        .collect();
    for h in handles {
        println!("{}", h.join().unwrap());
    }

    // Channels: threads send messages instead of sharing memory.
    let (tx, rx) = mpsc::channel();
    for id in 0..3 {
        let tx = tx.clone();
        thread::spawn(move || tx.send(id * 10).unwrap());
    }
    drop(tx); // close our copy so the loop below ends
    let mut got: Vec<i32> = rx.iter().collect();
    got.sort();
    println!("received {got:?}");

    // Arc<Mutex<T>>: shared, changeable data across threads.
    let counter = Arc::new(Mutex::new(0));
    let handles: Vec<_> = (0..8)
        .map(|_| {
            let counter = Arc::clone(&counter);
            thread::spawn(move || {
                for _ in 0..1000 {
                    *counter.lock().unwrap() += 1;
                }
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
    println!(
        "counter = {} (always 8000 — no data races)",
        *counter.lock().unwrap()
    );
}
