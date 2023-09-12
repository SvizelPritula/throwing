use proc_macro::TokenStream;

mod attributes;
mod types;

#[proc_macro]
pub fn define_error(attributes: TokenStream) -> TokenStream {
    TokenStream::new()
}
