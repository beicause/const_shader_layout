use core::num::{NonZero, Wrapping};
use glam::{
    I16Vec2, I16Vec3, I16Vec4, IVec2, IVec3, IVec4, Mat2, Mat3A, Mat4, Quat, U16Vec2, U16Vec3,
    U16Vec4, UVec2, UVec3, UVec4, Vec2, Vec3, Vec4,
};

pub trait ShaderType {
    const ALIGN: usize;
}

macro_rules! impl_raw_shader_type {
    ($($ty:ty),+$(,)?) => {
        $(impl ShaderType for $ty {
            const ALIGN: usize = size_of::<$ty>();
        })+
    };
}

macro_rules! impl_shader_type {
    ($align:expr $(, $ty:ty)+$(,)?) => {
        $(impl ShaderType for $ty {
            const ALIGN: usize = $align;
        })+
    };
}

// Scalar
impl_raw_shader_type!(
    half::f16,
    i16,
    u16,
    NonZero<i16>,
    NonZero<u16>,
    Wrapping<i16>,
    Wrapping<u16>,
    f32,
    i32,
    u32,
    NonZero<i32>,
    NonZero<u32>,
    Wrapping<f32>,
    Wrapping<i32>,
    Wrapping<u32>,
);

// Vector
impl_raw_shader_type!(
    I16Vec2, U16Vec2, I16Vec4, U16Vec4, IVec2, UVec2, Vec2, IVec4, UVec4, Vec4, Quat
);
impl_shader_type!(8, I16Vec3, U16Vec3);
impl_shader_type!(16, IVec3, UVec3, Vec3);

// Matrix
// Can't use `Mat3` as its column vectors are not properly aligned.
impl_raw_shader_type!(Mat2, Mat3A, Mat4);

// Array
impl<T: ShaderType, const N: usize> ShaderType for [T; N] {
    const ALIGN: usize = T::ALIGN;
}

#[macro_export]
macro_rules! shader_type {
    (
        $(#[$attr:meta])*
        $vis:vis struct $struct_name:ident {
            $(
                $(#[$field_attr:meta])*
                $field_vis:vis $field_name:ident : $field_ty:ty
            ),* $(,)?
        }
   ) => {
        $(#[$attr])*
        $vis struct $struct_name {
            $(
                $(#[$field_attr])*
                $field_vis $field_name: $field_ty
            ),*
        }

        $(
            const _ :() = {
                assert!(
                    size_of::<$field_ty>() != 0,
                    concat!(
                        "In a `shader_type!`, field `",
                        stringify!($struct_name), "::", stringify!($field_name),
                        "` size must not be 0",
                    ),
                );

                const OFFSET: usize = core::mem::offset_of!($struct_name, $field_name);
                const ALIGN: usize = <$field_ty as $crate::ShaderType>::ALIGN;
                assert!(
                    OFFSET % ALIGN == 0,
                    concat!(
                        "In a `shader_type!`, field `",
                        stringify!($struct_name), "::", stringify!($field_name),
                        "` is not properly aligned",
                    ),
                );
            };
        )*

        const ALIGNS: &[usize] = &[$(
            (<$field_ty as $crate::ShaderType>::ALIGN)
        ),*];
        const MAX_ALIGN: usize = {
            let mut max = ALIGNS[0];
            let mut i = 1;
            while i < ALIGNS.len() {
                if ALIGNS[i] > max {
                    max = ALIGNS[i];
                }
                i += 1;
            }
            max
        };
        impl $crate::ShaderType for $struct_name {
            const ALIGN: usize = MAX_ALIGN;
        }
    };
}

shader_type! {
    #[derive(bytemuck::NoUninit, Copy, Clone)]
    #[repr(C)]
    pub struct MyUniform {
        a1: f32,
        a2: [f32; 2],
        a3: [f32; 1],
        a4: Vec3,
    }
}
