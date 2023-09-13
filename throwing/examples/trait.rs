use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    num::ParseIntError,
    str::FromStr,
};

use throwing_macros::{define_error, throws};

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct Fraction {
    numerator: u64,
    denominator: u64,
}

define_error!(pub type ParseFractionError = ParseIntError | BadLengthError);

impl FromStr for Fraction {
    type Err = ParseFractionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let components: Vec<&str> = s.split('/').collect();
        let components: [&str; 2] = components.try_into().map_err(|_| BadLengthError)?;

        let numerator = components[0].parse()?;
        let denominator = components[1].parse()?;

        Ok(Fraction {
            numerator,
            denominator,
        })
    }
}

#[throws(ParseFractionError)]
fn main() {
    let frac: Fraction = "22/7".parse()?;
    println!("{frac:?}");

    Ok(())
}

// A simple error like this can be conveniently created with thiserror

#[derive(Debug, Clone, Copy)]
pub struct BadLengthError;

impl Display for BadLengthError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "bad length")
    }
}

impl Error for BadLengthError {}
