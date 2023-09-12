use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Ident, Result, Token, Type, Visibility,
};

pub enum VariantArg {
    Variant { typ: Type, name: Option<Ident> },
    Composed { typ: Type },
}

impl Parse for VariantArg {
    fn parse(input: ParseStream) -> Result<Self> {
        let break_tok: Option<Token!(break)> = input.parse()?;

        if break_tok.is_some() {
            let typ: Type = input.parse()?;
            Ok(VariantArg::Composed { typ })
        } else {
            let typ: Type = input.parse()?;
            let as_tok: Option<Token!(as)> = input.parse()?;

            let name: Option<Ident> = if as_tok.is_some() {
                Some(input.parse()?)
            } else {
                None
            };

            Ok(VariantArg::Variant { typ, name })
        }
    }
}

pub struct TypeDef {
    pub visibility: Visibility,
    pub name: Ident,
}

impl Parse for TypeDef {
    fn parse(input: ParseStream) -> Result<Self> {
        let visibility = input.parse()?;
        input.parse::<Token!(type)>()?;
        let name = input.parse()?;

        Ok(TypeDef { visibility, name })
    }
}

pub type VariantArgs = Punctuated<VariantArg, Token!(|)>;

pub struct DefineErrorArgs {
    pub type_def: TypeDef,
    pub variants: VariantArgs,
}

impl Parse for DefineErrorArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let type_def = input.parse()?;
        let equal: Option<Token!(=)> = input.parse()?;

        let variants = if equal.is_some() {
            input.parse_terminated(VariantArg::parse, Token!(|))?
        } else {
            VariantArgs::default()
        };

        Ok(DefineErrorArgs { type_def, variants })
    }
}
