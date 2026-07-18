#![no_std]

use core::num::{NonZero, Wrapping};
#[cfg(any(feature = "glam", feature = "glam_32"))]
mod glam;
#[cfg(feature = "half")]
mod half;

/// Marks the type's alignment and size in shader layout.
pub trait ShaderLayout: Clone + Copy + 'static {
    const ALIGN: NonZero<u64>;
    const SIZE: NonZero<u64>;
}

macro_rules! impl_shader_layout_raw {
    ($($ty:ty),+$(,)?) => {
        $(impl $crate::ShaderLayout for $ty {
            const ALIGN: ::core::num::NonZero<u64> = ::core::num::NonZero::new(align_of::<$ty>() as u64).unwrap();
            const SIZE: ::core::num::NonZero<u64> = ::core::num::NonZero::new(size_of::<$ty>() as u64).unwrap();
        })+
    };
}
#[cfg_attr(
    not(feature = "half"),
    expect(unused, reason = "This is only used across modules with some features")
)]
pub(crate) use impl_shader_layout_raw;

#[cfg_attr(
    not(feature = "glam"),
    expect(unused, reason = "This is only used with some features")
)]
macro_rules! impl_shader_layout {
    ($align:expr, $size:expr $(, $ty:ty)+$(,)?) => {
        $(impl $crate::ShaderLayout for $ty {
            const ALIGN: ::core::num::NonZero<u64> = ::core::num::NonZero::new($align).unwrap();
            const SIZE: ::core::num::NonZero<u64> = ::core::num::NonZero::new($size).unwrap();
        })+
    };
}
#[cfg_attr(
    not(feature = "glam"),
    expect(unused, reason = "This is only used across modules with some features")
)]
pub(crate) use impl_shader_layout;

macro_rules! impl_shader_layout_array {
    ($($ty:ty),+$(,)?) => {
        $(
            impl<const N: usize> $crate::ShaderLayout for [$ty; N]
            {
                const ALIGN: ::core::num::NonZero<u64> = <$ty as $crate::ShaderLayout>::ALIGN;
                const SIZE: ::core::num::NonZero<u64> = ::core::num::NonZero::new(
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
#[cfg_attr(
    all(not(feature = "glam"), not(feature = "half")),
    expect(unused, reason = "This is only used across modules with some features")
)]
pub(crate) use impl_shader_layout_array;

// Scalar
impl_shader_layout_raw!(
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

// Array
impl_shader_layout_array!(
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

/// Checks if all the struct's fields conform to shader layout then implements [`ShaderLayout`] for this struct, or fails at compile-time.
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
