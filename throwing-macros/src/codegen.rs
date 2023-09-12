use crate::types::{CompositeError, Variant};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Type;

fn error_enum(error: &CompositeError) -> TokenStream {
    let CompositeError {
        visibility,
        name,
        variants,
        ..
    } = error;

    let variants = variants
        .iter()
        .map(|Variant { name, typ }| quote!(#name(#typ)));

    quote!(
        #visibility enum #name {
            #(#variants),*
        }
    )
}

fn impl_from_variant(error: &CompositeError, variant: &Variant) -> TokenStream {
    let error_name = &error.name;
    let Variant { typ, name } = variant;

    quote!(
        impl ::std::convert::From<#typ> for #error_name {
            fn from(value: #typ) -> #error_name {
                #error_name::#name(value)
            }
        }
    )
}

fn impl_from_composed(error: &CompositeError, typ: &Type) -> TokenStream {
    let error_name = &error.name;

    quote!(
        impl ::std::convert::From<#typ> for #error_name {
            fn from(value: #typ) -> #error_name {
                ::throwing::SubError::to_super_error(value)
            }
        }
    )
}

fn impl_sub_error(error: &CompositeError) -> TokenStream {
    let CompositeError { name, variants, .. } = error;

    let froms = variants
        .iter()
        .map(|Variant { typ, .. }| quote!(::std::convert::From<#typ>));

    let arms = variants
        .iter()
        .map(|Variant { name: variant, .. }| quote!(#name::#variant(e) => T::from(e)));

    quote!(
        impl<T> ::throwing::SubError<T> for #name where T: #(#froms)+* {
            fn to_super_error(self) -> T {
                match self {
                    #(#arms),*
                }
            }
        }
    )
}

pub fn error_definition(error: CompositeError) -> TokenStream {
    let mut stream = error_enum(&error);

    for variant in &error.variants {
        stream.extend(impl_from_variant(&error, variant));
    }

    for typ in &error.composed {
        stream.extend(impl_from_composed(&error, typ));
    }

    stream.extend(impl_sub_error(&error));

    stream
}
