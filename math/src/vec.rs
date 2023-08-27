use std::ops::{Sub, Add, Mul, Neg};
use super::MutF32;

macro_rules! strip_plus {
    (+ $($rest: tt)*) => {
        $($rest)*
    }
}
macro_rules! count {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + count!($($xs)*));
}

macro_rules! vec {
    ($name: ident { $($field:ident),* }) => {
        paste::paste! {
            #[derive(Default)]
            pub struct [<Mut$name>] {
                $( pub $field: MutF32 ),*
            }
            impl [<Mut$name>] {
                #[inline(always)]
                pub const fn new($( $field: f32 ),*) -> Self {
                    Self {
                        $( $field: MutF32::new($field) ),*
                    }
                }
                #[inline(always)]
                pub fn set(&self, $($field: f32),*) {
                    $( self.$field.set($field) );*
                }
                #[inline(always)]
                pub fn get(&self) -> $name {
                    $name {
                        $( $field: self.$field.get() ),*
                    }
                }
            }
            impl Into<[<Mut$name>]> for $name {
                fn into(self) -> [<Mut$name>] {
                    [<Mut$name>]::new($( self.$field ),*)
                }
            }

            #[repr(C)]
            #[derive(Debug, Default, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
            pub struct $name {
                $( pub $field: f32 ),*
            }
            impl $name {
                #[inline(always)]
                pub const fn new($( $field: f32 ),*) -> Self {
                    Self { $( $field ),* }
                }
                #[inline(always)]
                pub fn normalized(self) -> Self {
                    let mag = 1. / strip_plus!($(+(self.$field*self.$field))+).sqrt();
                    Self { $(
                        $field: self.$field * mag
                    ),* }
                }
                #[inline(always)]
                pub fn dot(self, other: Self) -> f32 {
                    strip_plus!($(+(self.$field*other.$field))+)
                }
            }
            impl From<f32> for $name {
                fn from(v: f32) -> Self {
                    Self {
                        $( $field: v ),*
                    }
                }
            }
            impl From<$name> for [f32;count!($($field)*)] {
                fn from(v: $name) -> Self {
                    [ $( v.$field ),* ]
                }
            }
            impl From<[f32;count!($($field)*)]> for $name {
                fn from(v: [f32;count!($($field)*)]) -> $name {
                    let mut iter = v.into_iter();
                    $name { $(
                        $field: iter.next().unwrap()
                    ),* }
                }
            }
            impl PartialEq<$name> for $name {
                fn eq(&self, v: &Self) -> bool {
                    $(
                        self.$field == v.$field
                    )&&*
                }
            }
            impl Sub<$name> for $name {
                type Output = Self;
                fn sub(self, rhs: Self) -> Self::Output {
                    Self {$(
                        $field: self.$field - rhs.$field
                    ),* }
                }
            }
            impl Add<$name> for $name {
                type Output = Self;
                fn add(self, rhs: Self) -> Self::Output {
                    Self {$(
                        $field: self.$field + rhs.$field
                    ),* }
                }
            }
            impl Mul<f32> for $name {
                type Output = Self;
                fn mul(self, rhs: f32) -> Self::Output {
                    Self {$(
                        $field: self.$field * rhs
                    ),* }
                }
            }
            impl Neg for $name {
                type Output = Self;
                fn neg(self) -> Self::Output {
                    Self {$(
                        $field: -self.$field
                    ),* }
                }
            }
        }
    };
}

vec!(Vec2 { x, y });
vec!(Vec3 { x, y, z });
vec!(Vec4 { x, y, z, w });

impl Vec3 {
    #[inline(always)]
    pub fn cross(self, v: Self) -> Self {
        Self {
            x: (self.y * v.z) - (self.z * v.y),
            y: (self.z * v.x) - (self.x * v.z),
            z: (self.x * v.y) - (self.y * v.x)
        }
    }
    #[inline(always)]
    pub fn rotate_x(self, ang: f32) -> Self {
        let cos = ang.cos();
        let sin = ang.sin();
        Self {
            x: self.x,
            y: self.y * cos + self.z * sin,
            z: self.z * cos - self.y * sin
        }
    }
    #[inline(always)]
    pub fn rotate_y(self, ang: f32) -> Self {
        let cos = ang.cos();
        let sin = ang.sin();
        Self {
            x: self.x * cos - self.z * sin,
            y: self.y,
            z: self.x * sin + self.z * cos
        }
    }
    #[inline(always)]
    pub fn rotate_z(self, ang: f32) -> Self {
        let cos = ang.cos();
        let sin = ang.sin();
        Self {
            x: self.x * cos + self.y * sin,
            y: self.y * cos - self.x * sin,
            z: self.z
        }
    }
}