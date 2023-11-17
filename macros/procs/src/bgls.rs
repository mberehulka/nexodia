use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::{parse::ParseStream, parse_macro_input, Expr};

fn parse_visibility(v: &Expr) -> proc_macro2::TokenStream {
    let args = v.into_token_stream().to_string()
        .split('_')
        .map(|v| match v.trim() {
            "FRAGMENT" => quote!(wgpu::ShaderStages::FRAGMENT),
            "VERTEX" => quote!(wgpu::ShaderStages::VERTEX),
            v => panic!("Wrong visibility argument: {}", v)
        })
        .collect::<Vec<_>>();
    quote!( #(#args)|* )
}

pub struct Args {
    pub bgls: Vec<proc_macro2::TokenStream>
}
impl syn::parse::Parse for Args {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut bgls = Vec::new();
        let mut binding = 0u32;
        while !input.is_empty() {
            let _type = input.parse::<Ident>()?;
            
            let (visibility, arg1) = match input.parse::<Expr>()? {
                Expr::Tuple(v) => {
                    let mut args = v.elems.into_iter();
                    (
                        parse_visibility(&args.next().unwrap()),
                        args.next()
                    )
                },
                Expr::Paren(v) => (
                    parse_visibility(&v.expr),
                    None
                ),
                _ => panic!("Wrong visibility type")
            };

            bgls.push(match _type.to_string().as_str() {
                "Uniform" => quote!(
                    wgpu::BindGroupLayoutEntry {
                        binding: #binding,
                        visibility: #visibility,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None
                        },
                        count: None
                    }
                ),
                "TextureView" => quote!(
                    wgpu::BindGroupLayoutEntry {
                        binding: #binding,
                        visibility: #visibility,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::#arg1
                        },
                        count: None
                    }
                ),
                "TextureSampler" => quote!(
                    wgpu::BindGroupLayoutEntry {
                        binding: #binding,
                        visibility: #visibility,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::#arg1),
                        count: None
                    }
                ),
                _ => panic!("Wrong Bgl type")
            });

            binding += 1;
        }
        Ok(Self { bgls })
    }
}

pub fn bgls(inp: TokenStream) -> TokenStream {
    let Args { bgls } = parse_macro_input!(inp as Args);
    quote!(
        pub fn bind_group_layouts(device: &wgpu::Device) -> wgpu::BindGroupLayout {
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some(module_path!()),
                entries: &[
                    #(#bgls),*
                ]
            })
        }
    ).into()
}