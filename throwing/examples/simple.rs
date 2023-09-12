use std::{io, num::ParseIntError};

use throwing::define_error;

define_error!(type ReadConfigError = ParseIntError | io::Error);

fn main() {}
