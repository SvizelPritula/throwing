use syn::{Ident, Type, Visibility};

pub struct CompositeError {
    visibility: Visibility,
    name: Ident,
    variants: Vec<Variant>,
    composed: Vec<Type>,
}

pub struct Variant {
    subtype: Type,
    name: Ident,
}
