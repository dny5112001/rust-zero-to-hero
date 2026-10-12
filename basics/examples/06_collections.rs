// Chapter 06 — Vec, String, HashMap.
use std::collections::HashMap;

fn main() {
    // Vec: a growable list
    let mut scores = vec![70, 85, 92];
    scores.push(60);
    println!(
        "scores = {scores:?}, first = {}, len = {}",
        scores[0],
        scores.len()
    );
    println!("get(10) = {:?}", scores.get(10)); // None instead of crashing
    scores.sort();
    println!("sorted = {scores:?}");

    // String
    let mut greeting = String::from("Hello");
    greeting.push_str(", world");
    let shout = greeting.to_uppercase();
    println!(
        "{greeting} / {shout} / contains 'world'? {}",
        greeting.contains("world")
    );
    for word in "the quick brown fox".split_whitespace() {
        print!("[{word}] ");
    }
    println!();

    // HashMap: key -> value
    let mut stock: HashMap<String, u32> = HashMap::new();
    stock.insert("apples".to_string(), 5);
    stock.insert("pears".to_string(), 2);
    *stock.entry("apples".to_string()).or_insert(0) += 3;
    stock.entry("plums".to_string()).or_insert(10);
    println!("apples = {:?}", stock.get("apples"));

    // Count words
    let text = "a rose is a rose is a rose";
    let mut counts: HashMap<&str, usize> = HashMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word).or_insert(0) += 1;
    }
    let mut pairs: Vec<_> = counts.into_iter().collect();
    pairs.sort();
    println!("word counts = {pairs:?}");
}
