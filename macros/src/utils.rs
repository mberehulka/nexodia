#[macro_export]
macro_rules! join_modules {
    (
        $struct_name: ident {
            $($child: ident: $module: ident)*
        }
    ) => {
        $(pub mod $child;)*
        pub struct $struct_name {
            $(pub $child: $child::$module),*
        }
        impl $struct_name {
            pub fn new(e: &'static engine::Engine) -> Self {
                Self {
                    $($child: $child::$module::new(e)),*
                }
            }
        }
    };
}