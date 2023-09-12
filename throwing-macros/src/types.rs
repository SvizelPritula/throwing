use syn::{Ident, Type, Visibility};

pub struct CompositeError {
    pub visibility: Visibility,
    pub name: Ident,
    pub variants: Vec<Variant>,
    pub composed: Vec<Type>,
}

pub struct Variant {
    pub typ: Type,
    pub name: Ident,
}
