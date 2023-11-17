extern crate proc_macro;
extern crate syn;
extern crate quote;
extern crate proc_macro2;

mod utils;
mod shader;
mod bgls;
mod cbg;
mod assets;

#[proc_macro]
pub fn shader(inp: proc_macro::TokenStream) -> proc_macro::TokenStream {
    shader::shader(inp)
}

#[proc_macro]
pub fn bind_group_layouts(inp: proc_macro::TokenStream) -> proc_macro::TokenStream {
    bgls::bgls(inp)
}

#[proc_macro]
pub fn create_bind_group(inp: proc_macro::TokenStream) -> proc_macro::TokenStream {
    cbg::create_bind_group(inp)
}

#[proc_macro]
pub fn assets(inp: proc_macro::TokenStream) -> proc_macro::TokenStream {
    assets::assets(inp)
}