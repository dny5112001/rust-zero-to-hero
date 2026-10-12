// Chapter 07 — errors with Result and `?`.
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
enum AgeError {
    NotANumber(ParseIntError),
    TooOld(u32),
}

impl fmt::Display for AgeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AgeError::NotANumber(e) => write!(f, "not a number: {e}"),
            AgeError::TooOld(age) => write!(f, "{age} is not a believable age"),
        }
    }
}

// Lets `?` convert a ParseIntError into our AgeError automatically.
impl From<ParseIntError> for AgeError {
    fn from(e: ParseIntError) -> Self {
        AgeError::NotANumber(e)
    }
}

fn parse_age(input: &str) -> Result<u32, AgeError> {
    let age: u32 = input.trim().parse()?; // on error: return early with AgeError
    if age > 130 {
        return Err(AgeError::TooOld(age));
    }
    Ok(age)
}

fn main() {
    for input in ["42", "abc", "200"] {
        match parse_age(input) {
            Ok(age) => println!("{input:>4} -> ok, age {age}"),
            Err(e) => println!("{input:>4} -> error: {e}"),
        }
    }
    // unwrap/expect crash on error — fine in tests and examples, avoid in real code.
    let sure = parse_age("30").expect("30 is valid");
    println!("sure = {sure}");
}
