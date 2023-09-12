use attributes::{DefineErrorArgs, VariantArg, VariantArgs};
use names::type_to_variant;
use proc_macro::TokenStream;
use syn::{parse_macro_input, Error, Type};
use types::{CompositeError, Variant};

mod attributes;
mod names;
mod types;

#[proc_macro]
pub fn define_error(attributes: TokenStream) -> TokenStream {
    let attrs = parse_macro_input!(attributes as DefineErrorArgs);
    let DefineErrorArgs { type_def, variants } = attrs;

    let (variants, composed) = match split_variants(variants) {
        Ok(v) => v,
        Err(e) => return e.to_compile_error().into(),
    };

    let error = CompositeError {
        name: type_def.name,
        visibility: type_def.visibility,
        variants,
        composed,
    };

    TokenStream::new()
}

fn split_variants(args: VariantArgs) -> Result<(Vec<Variant>, Vec<Type>), Error> {
    let mut variants = Vec::new();
    let mut composed = Vec::new();

    for arg in args {
        match arg {
            VariantArg::Variant { typ, name } => {
                let name = name.or_else(|| type_to_variant(&typ)).ok_or_else(|| {
                    Error::new_spanned(
                        &typ,
                        "variant name can only be infered if the type is a path",
                    )
                })?;

                variants.push(Variant { typ, name })
            }
            VariantArg::Composed { typ } => composed.push(typ),
        }
    }

    Ok((variants, composed))
}
