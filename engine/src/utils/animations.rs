#[macro_export]
macro_rules! load_animations {
    (
        $struct_name: ident,
        $({
            $path: literal,
            $name: ident,
            $reset_start_position: expr
        })*
    ) => {
        pub struct $struct_name {
            $($name: &'static engine::Animation),*
        }
        impl $struct_name {
            pub fn new(e: &'static engine::Engine) -> Self {
                $(
                    let $name = Box::leak(Box::new(e.load_animation(
                        std::path::Path::new("assets/").join($path).join(stringify!($name)).with_extension("bin")
                    )));
                    if $reset_start_position { $name.reset_start_position() }
                )*
                Self {
                    $($name),*
                }
            }
        }
    };
}
pub use load_animations;