use crate::types::{CompositeError, Variant};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{parse_quote, ItemFn, ReturnType, Type};

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
        #[derive(::core::fmt::Debug)]
        #visibility enum #name {
            #(#variants),*
        }
    )
}

fn impl_from_variant(error: &CompositeError, variant: &Variant) -> TokenStream {
    let error_name = &error.name;
    let Variant { typ, name } = variant;

    quote!(
        #[automatically_derived]
        impl ::core::convert::From<#typ> for #error_name {
            fn from(value: #typ) -> #error_name {
                #error_name::#name(value)
            }
        }
    )
}

fn impl_from_composed(error: &CompositeError, typ: &Type) -> TokenStream {
    let error_name = &error.name;

    quote!(
        #[automatically_derived]
        impl ::core::convert::From<#typ> for #error_name {
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
        .map(|Variant { typ, .. }| quote!(::core::convert::From<#typ>));

    let arms = variants.iter().map(
        |Variant { name: variant, .. }| quote!(#name::#variant(e) => ::core::convert::From::from(e)),
    );

    quote!(
        #[automatically_derived]
        impl<T> ::throwing::SubError<T> for #name where T: #(#froms)+* {
            fn to_super_error(self) -> T {
                match self {
                    #(#arms),*
                }
            }
        }
    )
}

fn impl_display(error: &CompositeError) -> TokenStream {
    let CompositeError { name, variants, .. } = error;

    let body = if variants.is_empty() {
        quote!(match *self {})
    } else {
        let arms = variants
            .iter()
            .map(|Variant { name: variant, .. }| quote!(#name::#variant(e) => ::core::fmt::Display::fmt(e, f)));

        quote!(
            match self {
                #(#arms),*
            }
        )
    };

    quote!(
        #[automatically_derived]
        impl ::core::fmt::Display for #name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                #body
            }
        }
    )
}

fn impl_error(error: &CompositeError) -> TokenStream {
    let CompositeError { name, variants, .. } = error;

    let body = if variants.is_empty() {
        quote!(match *self {})
    } else {
        let arms = variants
            .iter()
            .map(|Variant { name: variant, .. }| quote!(#name::#variant(e) => ::core::option::Option::Some(e)));

        quote!(
            match self {
                #(#arms),*
            }
        )
    };

    quote!(
        #[automatically_derived]
        impl ::std::error::Error for #name {
            fn source(&self) -> ::core::option::Option<&(dyn ::std::error::Error + 'static)> {
                #body
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
    stream.extend(impl_display(&error));
    stream.extend(impl_error(&error));

    stream
}

fn wrap_return_with_result(ret: ReturnType, error: Ident) -> ReturnType {
    let (arrow, typ) = match ret {
        ReturnType::Default => (Default::default(), parse_quote!(())),
        ReturnType::Type(arrow, typ) => (arrow, *typ),
    };

    let typ = parse_quote!(::core::result::Result<#typ, #error>);

    ReturnType::Type(arrow, Box::new(typ))
}

pub fn patch_function(mut function: ItemFn, error: Ident) -> ItemFn {
    function.sig.output = wrap_return_with_result(function.sig.output, error);

    function
}
