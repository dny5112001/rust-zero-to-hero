// Chapter 04 — ownership, moves, borrowing.
fn main() {
    // A String owns heap memory. Assigning MOVES ownership.
    let a = String::from("hello");
    let b = a; // `a` is no longer usable
    // println!("{a}"); // <- uncomment: compile error "borrow of moved value"
    println!("b = {b}");

    // clone() makes a real copy (costs memory/time).
    let c = b.clone();
    println!("b = {b}, c = {c}");

    // Simple values (numbers, bool, char) are Copy: they are copied, not moved.
    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}");

    // Passing to a function moves too...
    takes_ownership(c);
    // ...so borrow instead with & (read-only) to keep using it.
    let word = String::from("borrowed");
    print_len(&word);
    println!("still mine: {word}");

    // &mut = a mutable borrow. Only ONE at a time, and no readers meanwhile.
    let mut list = vec![1, 2, 3];
    add_item(&mut list);
    println!("list = {list:?}");

    // Slices borrow part of something.
    let sentence = String::from("hello world");
    println!("first word = {}", first_word(&sentence));
}

fn takes_ownership(s: String) {
    println!("I own '{s}' now; it's freed when this function ends");
}

fn print_len(s: &str) {
    println!("'{s}' has {} bytes", s.len());
}

fn add_item(list: &mut Vec<i32>) {
    list.push(4);
}

fn first_word(s: &str) -> &str {
    s.split(' ').next().unwrap_or("")
}
