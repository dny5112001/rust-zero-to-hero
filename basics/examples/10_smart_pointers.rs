// Chapter 10 — Box, Rc, RefCell, lifetimes.
use std::cell::RefCell;
use std::rc::Rc;

// Box: put a value on the heap. Needed for recursive types.
#[derive(Debug)]
enum List {
    Item(i32, Box<List>),
    End,
}

fn sum(list: &List) -> i32 {
    match list {
        List::Item(value, rest) => value + sum(rest),
        List::End => 0,
    }
}

// Lifetimes: "the returned reference lives as long as both inputs".
fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

fn main() {
    let list = List::Item(1, Box::new(List::Item(2, Box::new(List::End))));
    println!("{list:?} sums to {}", sum(&list));

    // Rc: several owners of the same value (single thread). Freed when the last owner goes.
    let shared = Rc::new(String::from("shared config"));
    let a = Rc::clone(&shared);
    let b = Rc::clone(&shared);
    println!("{a} — owners: {}", Rc::strong_count(&b));

    // RefCell: change something through a shared reference, checked at runtime.
    let cart = Rc::new(RefCell::new(Vec::new()));
    let cart_for_user = Rc::clone(&cart);
    cart_for_user.borrow_mut().push("book");
    cart.borrow_mut().push("pen");
    println!("cart = {:?}", cart.borrow());

    println!("longer = {}", longer("hello", "hi"));
}
