#[macro_export]
macro_rules! shaders {
    ($($name: ident),*) => {
        use engine::Shader;
        $( pub mod $name; )*
        pub struct Shaders {
            $( pub $name: $name::Shader ),*
        }
        impl Shaders {
            pub fn new(e: &'static engine::Engine) -> Self {
                let start = std::time::Instant::now();
                let s = Self {
                    $( $name: $name::Shader::new(e) ),*
                };
                info!("Shaders compiled in: {}ms", (std::time::Instant::now() - start).as_millis());
                s
            }
        }
    };
}
pub use shaders;