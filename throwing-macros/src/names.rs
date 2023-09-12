use syn::{spanned::Spanned, Ident, Path, Type};

pub fn type_to_variant(typ: &Type) -> Option<Ident> {
    match typ {
        Type::Path(path) => {
            if path.qself.is_none() {
                Some(path_to_variant(&path.path))
            } else {
                None
            }
        }
        Type::Paren(paren) => type_to_variant(&paren.elem),
        Type::Group(group) => type_to_variant(&group.elem),
        _ => None,
    }
}

fn path_to_variant(path: &Path) -> Ident {
    if path.segments.len() == 1 {
        return path.segments[0].ident.clone();
    }

    let mut name = String::new();

    for segment in &path.segments {
        let segment = segment.ident.to_string();
        let mut chars = segment.chars();

        if let Some(c) = chars.next() {
            name.extend(c.to_uppercase());
        }

        name.extend(chars);
    }

    Ident::new(&name, path.span())
}
