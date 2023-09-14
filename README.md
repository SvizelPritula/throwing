# throwing

This crate implements a `#[throws(...)]` macro that allows you to easily
declare what errors a function can return.
This will allow you to exhaustively match on all possible errors.
It is inspired by [declared exceptions][java-throws] in Java.

The `#[throws(...)]` macro will automatically generate an enum that can
represent all declared errors, generate `From<T>` implementations for
each variant, and change the return type of the function to
an appropriate `Result<T, E>`.
The error type will also have implementations of `Error`, `Display` and `Debug`.

Additionally, it can generate `From<T>` implementation for upcasting errors,
that is converting an error type with fewer variants to one with more variants.

[java-throws]: https://docs.oracle.com/javase/tutorial/essential/exceptions/declaring.html

## Installation

You can add this crate to your project with: 

```sh
cargo add throwing
```

## Examples

### Fetching information about rabbits from Wikipedia

```rust
use std::io::{self, stdout, Write};
use serde::Deserialize;
use throwing::throws;

#[derive(Clone, Deserialize)]
struct Summary {
    extract: String,
}

#[throws(reqwest::Error | serde_json::Error)]
fn fetch_extract() -> String {
    let url = "https://en.wikipedia.org/api/rest_v1/page/summary/Rabbit";
    let response = reqwest::blocking::get(url)?;

    let summary = response.text()?;
    let summary: Summary = serde_json::from_str(&summary)?;

    Ok(summary.extract)
}

#[throws(reqwest::Error | serde_json::Error | io::Error | break FetchExtractError)]
fn main() {
    let extract = fetch_extract()?;
    writeln!(stdout(), "{extract}")?;

    Ok(())
}
```

### Reading an integer from a file and handling errors

```rust
use std::{fs, io, num::ParseIntError};

use throwing::throws;

#[throws(ParseIntError | io::Error)]
fn read_int_from_file(path: &str) -> i64 {
    let content = fs::read_to_string(path)?;
    let value = content.trim().parse()?;
    Ok(value)
}

fn main() {
    match read_int_from_file("file.txt") {
        Ok(number) => println!("{number}"),
        Err(ReadIntFromFileError::ParseIntError(e)) => eprintln!("Failed to parse int: {e}"),
        Err(ReadIntFromFileError::IoError(e)) => eprintln!("Failed to read file: {e}"),
    }
}
```

### Implementing FromStr for a custom type

```rust
use std::{num::ParseIntError, str::FromStr};
use throwing::define_error;

pub struct Id(u64);

define_error!(pub type ParseIdError = ParseIntError);

impl FromStr for Id {
    type Err = ParseIdError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.parse()?;
        Ok(value)
    }
}
```

### Adding integers with custom errors through thiserror

You can use [thiserror](https://docs.rs/thiserror/latest/thiserror/) to easily make simple error types.

```rust
use std::{
    io::{self, stdin, BufRead, Lines},
    num::ParseIntError,
};

use thiserror::Error;
use throwing::throws;

#[derive(Debug, Error)]
#[error("unexpected end of file")]
struct EofError;

#[derive(Debug, Error)]
#[error("addition of {0} and {1} overflows")]
struct OverflowError(i32, i32);

fn add(a: i32, b: i32) -> Result<i32, OverflowError> {
    a.checked_add(b).ok_or_else(|| OverflowError(a, b))
}

#[throws(io::Error | EofError)]
fn read_line(input: &mut Lines<impl BufRead>) -> String {
    Ok(input.next().ok_or(EofError)??)
}

#[throws(io::Error | EofError | ParseIntError | OverflowError | break ReadLineError)]
fn main() {
    let mut input = stdin().lock().lines();

    let a = read_line(&mut input)?.parse()?;
    let b = read_line(&mut input)?.parse()?;

    let result = add(a, b)?;
    println!("{result}");

    Ok(())
}
```
