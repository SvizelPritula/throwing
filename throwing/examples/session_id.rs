/// Reads the current session id on Linux

use std::{fs, io, num::ParseIntError, string::FromUtf8Error};

use throwing::throws;

#[throws(FromUtf8Error | ParseIntError)]
fn parse_int_from_bytes(payload: Vec<u8>) -> u64 {
    let string = String::from_utf8(payload)?;
    Ok(string.parse()?)
}

#[throws(io::Error | FromUtf8Error | ParseIntError | break ParseIntFromBytesError)]
fn get_session_id() -> u64 {
    let payload = fs::read("/proc/self/sessionid")?;
    Ok(parse_int_from_bytes(payload)?)
}

fn main() {
    match get_session_id() {
        Ok(id) => println!("{id}"),
        Err(GetSessionIdError::FromUtf8Error(_)) => eprintln!("File has invalid UTF-8"),
        Err(GetSessionIdError::ParseIntError(e)) => eprintln!("File contains invalid integer: {e}"),
        Err(GetSessionIdError::IoError(e)) => eprintln!("Failed to read file: {e}"),
    }
}
