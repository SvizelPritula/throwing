#![no_std]

pub use throwing_macros::{define_error, throws};

pub trait SubError<T> {
    fn to_super_error(self) -> T;
}
