// Chapter 09 — closures and iterators.
fn main() {
    // A closure is a small unnamed function that can use variables around it.
    let tax = 0.18;
    let with_tax = |price: f64| price * (1.0 + tax);
    println!("100 with tax = {:.2}", with_tax(100.0));

    let prices = [120.0, 45.5, 300.0, 9.99];

    // Iterator chains: describe WHAT you want, step by step. Nothing runs until
    // a "consumer" like collect/sum/count is called.
    let expensive: Vec<f64> = prices.iter().copied().filter(|p| *p > 50.0).collect();
    let total: f64 = prices.iter().sum();
    let with_tax_all: Vec<String> = prices
        .iter()
        .map(|p| format!("{:.2}", with_tax(*p)))
        .collect();
    println!("expensive = {expensive:?}");
    println!("total = {total:.2}");
    println!("with tax = {with_tax_all:?}");

    let names = ["asha", "ravi", "meera"];
    for (i, name) in names.iter().enumerate() {
        println!("{i}: {name}");
    }
    let shout: Vec<String> = names.iter().map(|n| n.to_uppercase()).collect();
    let has_long = names.iter().any(|n| n.len() > 4);
    let first_r = names.iter().find(|n| n.starts_with('r'));
    println!("{shout:?} any long name? {has_long} first starting with r: {first_r:?}");

    // Sum of squares of even numbers from 1 to 10
    let answer: u32 = (1..=10).filter(|n| n % 2 == 0).map(|n| n * n).sum();
    println!("sum of even squares = {answer}");

    // A closure that changes captured state needs `mut`.
    let mut counter = 0;
    let mut tick = || counter += 1;
    tick();
    tick();
    println!("counter = {counter}");
}
