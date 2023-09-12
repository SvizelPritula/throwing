use attributes::{DefineErrorArgs, ThrowsArgs, VariantArg, VariantArgs};
use codegen::{error_definition, patch_function};
use names::{fn_name_to_error, type_to_variant};
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, Error, Item, Type};
use types::{CompositeError, Variant};

mod attributes;
mod codegen;
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

    error_definition(error).into()
}

#[proc_macro_attribute]
pub fn throws(attributes: TokenStream, body: TokenStream) -> TokenStream {
    let body = parse_macro_input!(body as Item);
    let Item::Fn(function) = body else {
        return Error::new_spanned(body, "the throws macro can only be used on functions")
            .into_compile_error()
            .into();
    };

    let attrs = parse_macro_input!(attributes as ThrowsArgs);
    let ThrowsArgs { name, variants } = attrs;

    let name = name.unwrap_or_else(|| fn_name_to_error(&function.sig.ident));

    let (variants, composed) = match split_variants(variants) {
        Ok(v) => v,
        Err(e) => return e.to_compile_error().into(),
    };

    let error = CompositeError {
        name,
        visibility: function.vis.clone(),
        variants,
        composed,
    };

    let mut stream = patch_function(function, error.name.clone()).to_token_stream();
    stream.extend(error_definition(error));

    stream.into()
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
