use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::ParseStream, parse_macro_input, Expr};

pub struct Args {
    pub bgls: Expr,
    pub args: Vec<Expr>
}
impl syn::parse::Parse for Args {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let bgls = input.parse::<Expr>()?;
        let mut args = Vec::new();
        while !input.is_empty() {
            args.push(input.parse::<Expr>()?)
        }
        Ok(Self {
            bgls,
            args
        })
    }
}

pub fn create_bind_group(inp: TokenStream) -> TokenStream {
    let Args { bgls, args } = parse_macro_input!(inp as Args);
    let args = args.into_iter().enumerate().map(|(i, v)| {
        quote!(
            wgpu::BindGroupEntry {
                binding: #i as u32,
                resource: #v
            }
        )
    }).collect::<Vec<_>>();
    quote!(
        e.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &#bgls,
            entries: &[ #(#args),* ]
        })
    ).into()
}