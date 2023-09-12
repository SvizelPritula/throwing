pub use throwing_macros::define_error;

pub trait SubError<T> {
    fn to_super_error(self) -> T;
}
