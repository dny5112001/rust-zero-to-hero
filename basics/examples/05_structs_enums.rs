// Chapter 05 — structs, enums, match, Option.
#[derive(Debug)]
struct User {
    name: String,
    age: u32,
}

impl User {
    fn new(name: &str, age: u32) -> Self {
        User {
            name: name.to_string(),
            age,
        }
    }
    fn is_adult(&self) -> bool {
        self.age >= 18
    }
    fn birthday(&mut self) {
        self.age += 1;
    }
}

#[derive(Debug)]
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Dot,
}

fn area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
        Shape::Rectangle { width, height } => width * height,
        Shape::Dot => 0.0,
    }
}

fn find_user<'a>(users: &'a [User], name: &str) -> Option<&'a User> {
    users.iter().find(|u| u.name == name)
}

fn main() {
    let mut asha = User::new("Asha", 17);
    println!("{asha:?} adult? {}", asha.is_adult());
    asha.birthday();
    println!("after birthday: adult? {}", asha.is_adult());

    for shape in [
        Shape::Circle { radius: 1.0 },
        Shape::Rectangle {
            width: 2.0,
            height: 3.0,
        },
        Shape::Dot,
    ] {
        println!("{shape:?} -> area {:.2}", area(&shape));
    }

    // Option: Some(value) or None — Rust has no null.
    let users = vec![asha, User::new("Ravi", 30)];
    match find_user(&users, "Ravi") {
        Some(user) => println!("found {} ({})", user.name, user.age),
        None => println!("not found"),
    }
    if let Some(user) = find_user(&users, "Nobody") {
        println!("found {}", user.name);
    } else {
        println!("Nobody is not here");
    }
    let age = find_user(&users, "Asha").map(|u| u.age).unwrap_or(0);
    println!("Asha's age = {age}");
}
