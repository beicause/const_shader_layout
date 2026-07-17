use core::num::{NonZero, Wrapping};
use glam::{
    I16Vec2, I16Vec3, I16Vec4, IVec2, IVec3, IVec4, Mat2, Mat3A, Mat4, Quat, U16Vec2, U16Vec3,
    U16Vec4, UVec2, UVec3, UVec4, Vec2, Vec3, Vec4,
};

pub trait ShaderLayout: Clone + Copy + 'static {
    const ALIGN: NonZero<u64>;
    const SIZE: NonZero<u64>;
}

macro_rules! impl_shader_layout_raw {
    ($($ty:ty),+$(,)?) => {
        $(impl ShaderLayout for $ty {
            const ALIGN: NonZero<u64> = NonZero::new(align_of::<$ty>() as u64).unwrap();
            const SIZE: NonZero<u64> = NonZero::new(size_of::<$ty>() as u64).unwrap();
        })+
    };
}

macro_rules! impl_shader_layout {
    ($align:expr, $size:expr $(, $ty:ty)+$(,)?) => {
        $(impl ShaderLayout for $ty {
            const ALIGN: NonZero<u64> = NonZero::new($align).unwrap();
            const SIZE: NonZero<u64> = NonZero::new($size).unwrap();
        })+
    };
}

// Scalar
impl_shader_layout_raw!(
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
impl_shader_layout_raw!(
    I16Vec2, U16Vec2, I16Vec4, U16Vec4, IVec2, UVec2, Vec2, IVec4, UVec4, Vec4, Quat
);
impl_shader_layout!(8, 6, I16Vec3, U16Vec3);
impl_shader_layout!(16, 12, IVec3, UVec3, Vec3);

// Matrix
// Can't use `Mat3` as its column vectors are not properly aligned.
impl_shader_layout_raw!(Mat2, Mat3A, Mat4);

// Array
macro_rules! impl_shader_layout_array {
    ($($ty:ty),+$(,)?) => {
        $(
            impl<const N: usize> $crate::ShaderLayout for [$ty; N]
            {
                const ALIGN: NonZero<u64> = <$ty as $crate::ShaderLayout>::ALIGN;
                const SIZE: NonZero<u64> = NonZero::new(
                    <$ty as $crate::ShaderLayout>::SIZE.get()
                        .next_multiple_of(<$ty as $crate::ShaderLayout>::ALIGN.get()) * N as u64
                ).unwrap();
            }

            // Assert array size is equal to `N × roundUp(AlignOf(E), SizeOf(E))`
            const _ : () = {
                assert!(
                    <[$ty; 1] as $crate::ShaderLayout>::SIZE.get() == size_of::<[$ty; 1]>() as u64,
                    concat!(
                        "Size of `[",
                        stringify!($ty),
                        "; N]` must be equal to its shader size, i.e. `N × roundUp(AlignOf(E), SizeOf(E))`",
                    ),
                );
            };
        )+
    };
}
impl_shader_layout_array!(
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
// Vec3 is not implemented, because total size of `[Vec3; N]` != `N × roundUp(AlignOf(E), SizeOf(E))`
impl_shader_layout_array!(
    I16Vec2, U16Vec2, I16Vec4, U16Vec4, IVec2, UVec2, Vec2, IVec4, UVec4, Vec4, Quat
);

#[macro_export]
macro_rules! shader_layout {
    (
        $(#[$attr:meta])*
        $vis:vis struct $struct_name:ident {
            $(
                $(#[$field_attr:meta])*
                $field_vis:vis $field_name:ident : $field_ty:ty
            ),* $(,)?
        }
   ) => {
        #[derive(Copy, Clone)]
        #[repr(C)]
        $(#[$attr])*
        $vis struct $struct_name {
            $(
                $(#[$field_attr])*
                $field_vis $field_name: $field_ty
            ),*
        }

        $(
            const _ : () = {
                const OFFSET: u64 = core::mem::offset_of!($struct_name, $field_name) as u64;
                const ALIGN: u64 = <$field_ty as $crate::ShaderLayout>::ALIGN.get();
                assert!(
                    OFFSET.is_multiple_of(ALIGN),
                    concat!(
                        "In a `shader_layout!`, field `",
                        stringify!($struct_name), "::", stringify!($field_name),
                        "` is not properly aligned",
                    ),
                );
            };
        )*

        impl $crate::ShaderLayout for $struct_name {
            const ALIGN: ::core::num::NonZero<u64> = {
                const MEMBER_ALIGNS: &[u64] = &[$(
                    (<$field_ty as $crate::ShaderLayout>::ALIGN.get())
                ),*];

                let mut max = MEMBER_ALIGNS[0];
                let mut i = 1;
                while i < MEMBER_ALIGNS.len() {
                    if MEMBER_ALIGNS[i] > max {
                        max = MEMBER_ALIGNS[i];
                    }
                    i += 1;
                }
                ::core::num::NonZero::new(max).unwrap()
            };
            const SIZE: ::core::num::NonZero<u64> = ::core::num::NonZero::new(
                (size_of::<$struct_name>() as u64).next_multiple_of(<$struct_name as $crate::ShaderLayout>::ALIGN.get())
            ).unwrap();
        }

        // Assert struct has no padding, i.e. size must be equal to `roundUp(AlignOf(S), justPastLastMember))`
        const _ : () = {
            assert!(
                (size_of::<$struct_name>() as u64) == <$struct_name as $crate::ShaderLayout>::SIZE.get(),
                concat!(
                "In a `shader_layout!`, struct `",
                stringify!($struct_name),
                "` size must be equal to its shader size, i.e. `roundUp(AlignOf(S), justPastLastMember))`",
                ),
            );
        };
    };
}
