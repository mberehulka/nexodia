#[macro_export]
macro_rules! join_structs_f_args {
    (($($name: ident: $t: ty),*)) => {
        $($name),*
    }
}
pub use join_structs_f_args;

#[macro_export]
macro_rules! join_structs_plain_modules {
    ($name: ident, $($module: ident)::*) => {
        $name$(::$module)*
    }
}
pub use join_structs_plain_modules;

#[macro_export]
macro_rules! join_structs_plain_modules_call {
    ($name: ident, $($module: ident)::*, $f:ident, $dec: tt) => {
        $name$(::$module)*::$f(join_structs_f_args!($dec))
    }
}
pub use join_structs_plain_modules_call;

#[macro_export]
macro_rules! join_structs {
    ($name: ident [$($names: ident),*] {$modules: tt} $f:ident $dec: tt) => {
        $( pub mod $names; )*
        pub struct $name {
            $( pub $names: join_structs_plain_modules!($names, $modules) ),*
        }
        impl $name {
            pub fn $f $dec -> Self {
                Self {
                    $($names: join_structs_plain_modules_call!($names, $modules, $f, $dec)),*
                }
            }
        }
    }
}
pub use join_structs;