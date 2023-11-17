#[macro_export]
macro_rules! basic_material {
    (
        ($($arg_name: ident: $arg_type: ty),*)
        $cbg_expr: tt
        bind_group_index $bgi: expr
    ) => {
        pub struct Material(wgpu::BindGroup);
        impl Material {
            pub fn new($($arg_name: $arg_type),*) -> Self {
                Self($cbg_expr)
            }
        }
        impl engine::Material for Material {
            const BGI: u32 = $bgi;
            fn bind_group(&self) -> &wgpu::BindGroup { &self.0 }
        }
    };
}