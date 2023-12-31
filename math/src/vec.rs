use std::{
    ops::{Sub, Add, Mul, Div, Neg, MulAssign, AddAssign, SubAssign, DivAssign},
    sync::atomic::{AtomicU32, Ordering}, mem::transmute
};
use bincode::{Decode, Encode};

macro_rules! vec {
    ($name: ident { $($field:ident),* }) => {
        #[derive(Debug, Default, Copy, Clone, Encode, Decode)]
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
            #[inline(always)]
            pub fn distance(self, other: Self) -> f32 {
                self.dot(other).sqrt()
            }
            #[inline(always)]
            pub fn mul_element_wise(self, other: Self) -> Self {
                Self { $(
                    $field: self.$field * other.$field
                ),* }
            }
            #[inline(always)]
            pub fn sum(self) -> f32 {
                strip_plus!($(+(self.$field))+)
            }
            #[inline(always)]
            pub fn lerp(&mut self, other: Self, amount: f32) {
                $(
                    self.$field = self.$field + ((other.$field - self.$field) * amount)
                );*
            }
            paste::paste! {
                $(
                    #[inline(always)]
                    pub fn [<with_$field>](mut self, value: f32) -> Self {
                        self.$field = value;
                        self
                    }
                )*
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
        impl Div<$name> for $name {
            type Output = Self;
            fn div(self, rhs: Self) -> Self::Output {
                Self {$(
                    $field: self.$field / rhs.$field
                ),* }
            }
        }
        impl MulAssign for $name {
            fn mul_assign(&mut self, rhs: Self) {
                $(self.$field *= rhs.$field;)*
            }
        }
        impl AddAssign for $name {
            fn add_assign(&mut self, rhs: Self) {
                $(self.$field += rhs.$field;)*
            }
        }
        impl SubAssign for $name {
            fn sub_assign(&mut self, rhs: Self) {
                $(self.$field -= rhs.$field;)*
            }
        }
        impl DivAssign for $name {
            fn div_assign(&mut self, rhs: Self) {
                $(self.$field /= rhs.$field;)*
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
        impl Div<f32> for $name {
            type Output = Self;
            fn div(self, rhs: f32) -> Self::Output {
                Self {$(
                    $field: self.$field / rhs
                ),* }
            }
        }
        impl Add<f32> for $name {
            type Output = Self;
            fn add(self, rhs: f32) -> Self::Output {
                Self {$(
                    $field: self.$field + rhs
                ),* }
            }
        }
        impl Sub<f32> for $name {
            type Output = Self;
            fn sub(self, rhs: f32) -> Self::Output {
                Self {$(
                    $field: self.$field - rhs
                ),* }
            }
        }
        impl MulAssign<f32> for $name {
            fn mul_assign(&mut self, rhs: f32) {
                $(self.$field *= rhs;)*
            }
        }
        impl AddAssign<f32> for $name {
            fn add_assign(&mut self, rhs: f32) {
                $(self.$field += rhs;)*
            }
        }
        impl SubAssign<f32> for $name {
            fn sub_assign(&mut self, rhs: f32) {
                $(self.$field -= rhs;)*
            }
        }
        impl DivAssign<f32> for $name {
            fn div_assign(&mut self, rhs: f32) {
                $(self.$field /= rhs;)*
            }
        }
        impl Mul<Self> for $name {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self {
                Self { $(
                    $field: self.$field * rhs.$field
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
        paste::paste! {
            #[allow(non_camel_case_types)]
            enum [<$name Indexes>] { $($field),* }
            #[allow(non_upper_case_globals)]
            const [<$name IndexesLength>]: usize = [$([<$name Indexes>]::$field as usize),*].len();

            impl From<[f32;[<$name IndexesLength>]]> for $name {
                fn from(v: [f32;[<$name IndexesLength>]]) -> Self {
                    $name::new(
                        $( *unsafe { v.get_unchecked([<$name Indexes>]::$field as usize) } ),*
                    )
                }
            }

            #[derive(Default)]
            pub struct [<M $name>] {
                $($field: AtomicU32),*
            }
            impl [<M $name>] {
                pub const fn new($($field: f32),*) -> Self {
                    Self {
                        $($field: unsafe { transmute($field) }),*
                    }
                }
                #[inline(always)]
                pub fn set(&self, v: f32) {
                    let v = unsafe { transmute(v) };
                    $(self.$field.store(v, Ordering::Relaxed));*
                }
                #[inline(always)]
                pub fn get(&self) -> $name {
                    $name::new($(self.$field()),*)
                }
                #[inline(always)]
                pub fn add(&self, v: f32) {
                    $(self.[<add_ $field>](v));*
                }
                #[inline(always)]
                pub fn sub(&self, v: f32) {
                    $(self.[<sub_ $field>](v));*
                }
                $(
                    #[inline(always)]
                    pub fn $field(&self) -> f32 {
                        unsafe { transmute(self.$field.load(Ordering::Relaxed)) }
                    }
                    #[inline(always)]
                    pub fn [<set_ $field>](&self, v: f32) {
                        self.$field.store(unsafe { transmute(v) }, Ordering::Relaxed)
                    }
                    #[inline(always)]
                    pub fn [<add_ $field>](&self, v: f32) {
                        self.[<set_ $field>](self.[<$field>]() + v)
                    }
                    #[inline(always)]
                    pub fn [<sub_ $field>](&self, v: f32) {
                        self.[<set_ $field>](self.[<$field>]() - v)
                    }
                )*
            }
        }
    };
}

vec!(Vec2 { x, y });
vec!(Vec3 { x, y, z });
vec!(Vec4 { x, y, z, w });

impl Vec3 {
    pub fn cross(self, v: Self) -> Self {
        Self {
            x: (self.y * v.z) - (self.z * v.y),
            y: (self.z * v.x) - (self.x * v.z),
            z: (self.x * v.y) - (self.y * v.x)
        }
    }
    pub fn rotate_x(self, ang: f32) -> Self {
        let cos = ang.cos();
        let sin = ang.sin();
        Self {
            x: self.x,
            y: self.y * cos + self.z * sin,
            z: self.z * cos - self.y * sin
        }
    }
    pub fn rotate_y(self, ang: f32) -> Self {
        let cos = ang.cos();
        let sin = ang.sin();
        Self {
            x: self.x * cos - self.z * sin,
            y: self.y,
            z: self.x * sin + self.z * cos
        }
    }
    pub fn rotate_z(self, ang: f32) -> Self {
        let cos = ang.cos();
        let sin = ang.sin();
        Self {
            x: self.x * cos + self.y * sin,
            y: self.y * cos - self.x * sin,
            z: self.z
        }
    }
    pub const fn extend(self, w: f32) -> Vec4 {
        Vec4::new(self.x, self.y, self.z, w)
    }
    pub fn truncate(self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
}
impl Vec4 {
    pub fn truncate(self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }
    pub fn truncate_n(self, n: u8) -> Vec3 {
        match n {
            0 => Vec3::new(self.y, self.z, self.w),
            1 => Vec3::new(self.x, self.z, self.w),
            2 => Vec3::new(self.x, self.y, self.w),
            3 => Vec3::new(self.x, self.y, self.z),
            _ => panic!("{:?} is out of range", n),
        }
    }
}
impl Vec2 {
    pub fn cross(self, v: Self) -> f32 {
        self.x * v.y - self.y * v.x
    }
}