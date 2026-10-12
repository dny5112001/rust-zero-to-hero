// Chapter 08 — traits and generics.
use std::fmt::Display;

trait Speak {
    fn name(&self) -> String;
    // A default method — types get it for free.
    fn greet(&self) -> String {
        format!("Hi, I'm {}", self.name())
    }
}

struct Dog;
struct Robot {
    id: u32,
}

impl Speak for Dog {
    fn name(&self) -> String {
        "Rex".into()
    }
    fn greet(&self) -> String {
        "Woof!".into() // override the default
    }
}

impl Speak for Robot {
    fn name(&self) -> String {
        format!("RX-{}", self.id)
    }
}

// Generic: works for ANY type T that implements Speak (decided at compile time).
fn introduce<T: Speak>(thing: &T) {
    println!("{}", thing.greet());
}

// Generic over anything that can be compared and printed.
fn largest<T: PartialOrd + Display + Copy>(items: &[T]) -> T {
    let mut best = items[0];
    for &item in items {
        if item > best {
            best = item;
        }
    }
    best
}

#[derive(Debug, Clone, PartialEq)] // derive = get common traits automatically
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    introduce(&Dog);
    introduce(&Robot { id: 7 });

    // A list of DIFFERENT types that share a trait: trait objects (Box<dyn Trait>).
    let crowd: Vec<Box<dyn Speak>> = vec![Box::new(Dog), Box::new(Robot { id: 1 })];
    for member in &crowd {
        println!("crowd: {}", member.greet());
    }

    println!("largest number = {}", largest(&[3, 9, 2]));
    println!("largest letter = {}", largest(&['x', 'b', 'q']));

    let p = Point { x: 1, y: 2 };
    let q = p.clone();
    println!("{p:?} == {q:?}? {}", p == q);
}
