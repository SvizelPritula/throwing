/// Adds newline seperated numbers from stdin

use std::{
    io::{self, stdin, stdout, BufRead, Write},
    num::ParseIntError,
};

use throwing_macros::throws;

#[throws(ParseIntError | io::Error)]
fn main() {
    let mut sum = 0u64;

    for line in stdin().lock().lines() {
        let line = line?;
        let value: u64 = line.parse()?;

        sum += value;
    }

    let mut output = stdout().lock();
    writeln!(output, "{sum}")?;

    Ok(())
}
