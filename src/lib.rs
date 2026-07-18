#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg), doc(auto_cfg = false))]
#![no_std]

use core::num::{NonZero, Wrapping};
#[cfg(feature = "glam")]
#[cfg_attr(docsrs, doc(cfg(feature = "glam")))]
mod glam;
#[cfg(feature = "half")]
#[cfg_attr(docsrs, doc(cfg(feature = "half")))]
mod half;

/// Marks the type's alignment requirement in shader.
///
/// Note: The `size_of::<T>` must be equal to its size in shader.
pub trait ShaderLayout: Clone + Copy + 'static {
    /// The type's alignment requirement in shader.
    const ALIGN: NonZero<u64>;
}

/// Implements [`ShaderLayout`] for the types, with their original alignment and size.
#[macro_export]
macro_rules! impl_shader_layout_raw {
    ($($ty:ty),+$(,)?) => {
        $(impl $crate::ShaderLayout for $ty {
            const ALIGN: ::core::num::NonZero<u64> = ::core::num::NonZero::new(align_of::<$ty>() as u64).unwrap();
        })+
    };
}

/// Implements [`ShaderLayout`] for the types, with the specified alignment and size.
#[macro_export]
macro_rules! impl_shader_layout {
    ($align:expr $(, $ty:ty)+$(,)?) => {
        $(impl $crate::ShaderLayout for $ty {
            const ALIGN: ::core::num::NonZero<u64> = ::core::num::NonZero::new($align).unwrap();
        })+
    };
}

/// Implements [`ShaderLayout`] for `[T; N]` for types implemented [`ShaderLayout`].
///
/// Checks at compile-time:
/// * Array size must be equal to `N × roundUp(AlignOf(E), SizeOf(E))`.
#[macro_export]
macro_rules! impl_shader_layout_array {
    ($($ty:ty),+$(,)?) => {
        $(
            impl<const N: usize> $crate::ShaderLayout for [$ty; N]
            {
                const ALIGN: ::core::num::NonZero<u64> = <$ty as $crate::ShaderLayout>::ALIGN;
            }

            // Assert array size is equal to `N × roundUp(AlignOf(E), SizeOf(E))`
            const _ : () = {
                const N: usize = 1;
                const SIZE: u64 = (size_of::<$ty>() as u64).next_multiple_of(<$ty as $crate::ShaderLayout>::ALIGN.get()) * N as u64;
                assert!(
                    SIZE == size_of::<[$ty; N]>() as u64,
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
///
/// Checks at compile-time:
/// * For each field, `core::mem::offset_of!(struct, field)` must be equal to its [`ShaderLayout::ALIGN`].
/// * Struct size must be equal to `roundUp(AlignOf(S), SizeOf(S)))`.
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
        }

        // Assert struct has no padding, i.e. size must be equal to `roundUp(AlignOf(S), justPastLastMember))`
        // `justPastLastMember` is equal to `size_of::<S>()` in `repr(C)`.
        const _ : () = {
            const SIZE: u64 = (size_of::<$struct_name>() as u64).next_multiple_of(<$struct_name as $crate::ShaderLayout>::ALIGN.get());
            assert!(
                (size_of::<$struct_name>() as u64) == SIZE,
                concat!(
                "In a `shader_layout!`, struct `",
                stringify!($struct_name),
                "` size must be equal to its shader size, i.e. `roundUp(AlignOf(S), SizeOf(S)))`",
                ),
            );
        };
    };
}
