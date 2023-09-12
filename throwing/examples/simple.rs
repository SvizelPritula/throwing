use std::{char::ParseCharError, io, num::ParseIntError};

use throwing::define_error;

define_error!(type ParseConfigError = ParseIntError | ParseCharError);
define_error!(type LoadConfigError = ParseIntError | ParseCharError | io::Error | break ParseConfigError);

fn main() {}
