use std::num::ParseIntError;

use throwing_macros::throws;

#[throws(type BadReverseInt = ParseIntError as ParseNormalIntError)]
fn parse_int_reversed(string: &str) -> u64 {
    let string: String = string.chars().rev().collect();
    Ok(string.parse()?)
}

fn main() {
    match parse_int_reversed("owt") {
        Ok(n) => println!("{n}"),
        Err(BadReverseInt::ParseNormalIntError(e)) => println!("{e}"),
    }
}
