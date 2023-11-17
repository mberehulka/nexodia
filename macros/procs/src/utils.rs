use quote::ToTokens;
use syn::{parse::ParseStream, Expr};
use proc_macro2::{Ident, TokenStream};

pub struct AnyToString(pub TokenStream);
impl syn::parse::Parse for AnyToString {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self(input.parse::<Expr>()?.to_token_stream()))
    }
}
impl ToTokens for AnyToString {
    fn into_token_stream(self) -> TokenStream where Self: Sized {
        self.0
    }
    fn to_token_stream(&self) -> TokenStream {
        self.0.clone()
    }
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.0.clone())
    }
}

pub fn assert_ident(input: ParseStream, v: &str) -> syn::Result<()> {
    if input.parse::<Ident>()?.to_string() != v {
        Err(input.error(format!("ident do not match: '{v}'")))
    } else {
        Ok(())
    }
}