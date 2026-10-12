// Chapter 03 — variables, types, functions, control flow.
fn main() {
    // Variables can't change unless you say `mut`.
    let apples = 5;
    let mut score = 0;
    score += 10;
    println!("apples={apples}, score={score}");

    // Shadowing: a new variable with the same name.
    let text = "42";
    let text: i32 = text.parse().expect("not a number");
    println!("text is now a number: {}", text + 1);

    // Basic types
    let price: f64 = 9.99;
    let in_stock: bool = true;
    let grade: char = 'A';
    let point: (i32, i32) = (3, 4); // tuple
    let days = ["Mon", "Tue", "Wed"]; // array (fixed size)
    println!("{price} {in_stock} {grade} {:?} {}", point, days[1]);

    // Functions: the last expression (no semicolon) is the return value.
    println!("area = {}", area(3, 4));

    // if is an expression
    let label = if score > 5 { "good" } else { "keep going" };
    println!("label = {label}");

    // loops
    for day in days {
        print!("{day} ");
    }
    println!();
    for i in 1..=3 {
        print!("{i} ");
    }
    println!();
    let mut n = 3;
    while n > 0 {
        n -= 1;
    }
    let mut tries = 0;
    let found = loop {
        tries += 1;
        if tries * tries > 40 {
            break tries; // loop can return a value with break
        }
    };
    println!("found = {found}");
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
