use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{
    parse::{ParseStream, ParseBuffer, Parse}, 
    token::{Gt, Slash},
    parse_macro_input, bracketed, Token, Expr
};

use crate::utils::assert_ident;

#[derive(Debug)]
pub struct Path(Vec<Ident>);
impl Parse for Path {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut values = Vec::new();
        while !input.is_empty() {
            values.push(Ident::parse(input)?);
            if input.peek(Token![/]) {
                Slash::parse(input)?;
            } else {
                break
            }
        }
        Ok(Self(values))
    }
}
impl Path {
    fn to_field(&self) -> Ident {
        Ident::new(
            &self.0.iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join("_"),
            Span::call_site()
        )
    }
    fn to_path_string(&self) -> String {
        self.0.iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join("/")
            .replace(' ', "")
    }
}

pub struct Paths(Vec<Path>);
impl Parse for Paths {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let paths: ParseBuffer;
        bracketed!(paths in input);
        Ok(Self (
            paths.parse_terminated(Path::parse, Token![,])?
                .into_iter().collect::<Vec<_>>()
        ))
    }
}

pub struct PathArg(Path, Expr);
impl Parse for PathArg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self(
            Path::parse(input)?,
            {   
                Gt::parse(input)?;
                Expr::parse(input)?
            }
        ))
    }
}

pub struct PathsArg(Vec<PathArg>);
impl Parse for PathsArg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let paths: ParseBuffer;
        bracketed!(paths in input);
        Ok(Self (
            paths.parse_terminated(PathArg::parse, Token![,])?
                .into_iter().collect::<Vec<_>>()
        ))
    }
}

pub struct Args {
    pub animations: Paths,
    pub meshes: PathsArg,
    pub textures: Paths
}
impl Parse for Args {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            animations: {
                assert_ident(input, "animations")?;
                Paths::parse(input)?
            },
            meshes: {
                assert_ident(input, "meshes")?;
                PathsArg::parse(input)?
            },
            textures: {
                assert_ident(input, "textures")?;
                Paths::parse(input)?
            }
        })
    }
}


pub fn assets(inp: TokenStream) -> TokenStream {
    let Args {
        animations: Paths ( animations ),
        meshes: PathsArg ( meshes ),
        textures: Paths ( textures )
    } = parse_macro_input!(inp as Args);

    let animations_paths = animations.iter().map(|path| path.to_path_string());
    let animations_fields = animations.iter().map(|path| path.to_field()).collect::<Vec<_>>();

    let meshes_paths = meshes.iter().map(|path_arg| path_arg.0.to_path_string());
    let meshes_fields = meshes.iter().map(|path_arg| path_arg.0.to_field()).collect::<Vec<_>>();
    let meshes_vertex_type = meshes.iter().map(|path_arg| &path_arg.1).collect::<Vec<_>>();

    let textures_paths = textures.iter().map(|path| path.to_path_string());
    let textures_fields = textures.iter().map(|path| path.to_field()).collect::<Vec<_>>();

    quote!(
        pub struct Assets {
            #(pub #animations_fields: engine::Animation,)*
            #(pub #meshes_fields: engine::Mesh<#meshes_vertex_type>,)*
            #(pub #textures_fields: engine::Texture,)*
        }
        impl Assets {
            pub fn new(e: &'static engine::Engine) -> Self {
                Self {
                    #(
                        #animations_fields: e.load_animation(
                            std::path::Path::new("assets/")
                                .join(#animations_paths)
                                .with_extension("bin")
                        ),
                    )*
                    #(
                        #meshes_fields: e.load_mesh::<#meshes_vertex_type>(
                            std::path::Path::new("assets/")
                                .join(#meshes_paths)
                                .with_extension("bin")
                        ),
                    )*
                    #(
                        #textures_fields: e.load_texture(
                            std::path::Path::new("assets/")
                                .join(#textures_paths)
                                .with_extension("bin")
                        ),
                    )*
                }
            }
        }
    ).into()
}