use core::num::NonZero;

use crate::ShaderLayout;

/// Marks the type's uniform-compatible alignment requirement in shader, i.e. with uniform address layout constraints.
///
/// Note: The `size_of::<T>` must be equal to its size in shader. Thus [`bool`] should not implement this.
///
/// See also <https://www.w3.org/TR/WGSL/#alignment-and-size> and <https://www.w3.org/TR/WGSL/#address-space-layout-constraints>
pub trait ShaderLayoutCompat: ShaderLayout {
    /// The type's alignment requirement with uniform address layout constraints in shader.
    const ALIGN_COMPAT: NonZero<u64> = Self::ALIGN;
}

/// Implements [`ShaderLayoutCompat`] (also implements [`ShaderLayout`]) for the types, with their original alignment.
#[macro_export]
macro_rules! impl_shader_layout_compat_raw {
    ($($ty:ty),+$(,)?) => {
        $(
            $crate::impl_shader_layout_raw!($ty);
            impl $crate::ShaderLayoutCompat for $ty {}
        )+
    };
}

/// Implements [`ShaderLayoutCompat`] (also implements [`ShaderLayout`]) for the types, with the specified alignment.
#[macro_export]
macro_rules! impl_shader_layout_compat {
    ($align:expr $(, $ty:ty)+$(,)?) => {
        $(
            $crate::impl_shader_layout!($align, $ty);
            impl $crate::ShaderLayoutCompat for $ty {}
        )+
    };
    ($align:expr, $align_compat:expr $(, $ty:ty)+$(,)?) => {
        $(
            $crate::impl_shader_layout!($align, $ty);
            impl $crate::ShaderLayoutCompat for $ty {
                const ALIGN_COMPAT: ::core::num::NonZero<u64> = ::core::num::NonZero::new($align_compat).unwrap();
            }
        )+
    };
}

/// Implements [`ShaderLayoutCompat`] (also implements [`ShaderLayout`]) for `[T; N]` for types implemented [`ShaderLayoutCompat`].
///
/// Different from [`ShaderLayout`], the stride of array must be a multiple of 16.
///
/// Checks at compile-time:
/// * Array size must be equal to `N × roundUp(16, roundUp(AlignOf(E), SizeOf(E)))`.
///
/// See also <https://www.w3.org/TR/WGSL/#alignment-and-size> and <https://www.w3.org/TR/WGSL/#address-space-layout-constraints>
#[cfg_attr(
    not(feature = "glam"),
    expect(unused_macros, reason = "unused without some features")
)]
macro_rules! impl_shader_layout_array_compat {
    ($($ty:ty),+$(,)?) => {
        $(
            $crate::impl_shader_layout_array!($ty);
            impl<const N: usize> $crate::ShaderLayoutCompat for [$ty; N]
            {
                const ALIGN_COMPAT: ::core::num::NonZero<u64> = ::core::num::NonZero::new(
                    <$ty as $crate::ShaderLayoutCompat>::ALIGN_COMPAT.get().next_multiple_of(16)
                ).unwrap();
            }

            // Assert array size is equal to `N × roundUp(16, roundUp(AlignOf(E), SizeOf(E)))`
            const _ : () = {
                const N: usize = 1;
                const SIZE: u64 = (
                    (size_of::<$ty>() as u64).next_multiple_of(<$ty as $crate::ShaderLayout>::ALIGN.get())
                    .next_multiple_of(16)
                ) * N as u64;
                assert!(
                    SIZE == size_of::<[$ty; N]>() as u64,
                    concat!(
                        "Size of `[",
                        stringify!($ty),
                        "; N]` must be equal to its shader size, i.e. `N × roundUp(16, roundUp(AlignOf(E), SizeOf(E)))`",
                        ", with uniform address layout constraints"
                    ),
                );
            };
        )+
    };
}
#[cfg_attr(
    not(feature = "glam"),
    expect(unused_imports, reason = "unused without some features")
)]
pub(crate) use impl_shader_layout_array_compat;

/// Checks if all the struct's fields conform to shader layout then implements [`ShaderLayoutCompat`] for this struct, or fails at compile-time.
///
/// Different from [`ShaderLayout`], the alignment of struct must be a multiple of 16.
///
/// Checks at compile-time:
/// * For each field, `core::mem::offset_of!(struct, field)` must be equal to its [`ShaderLayoutCompat::ALIGN_COMPAT`].
/// * Struct size must be equal to `roundUp(16, roundUp(AlignOf(S), SizeOf(S))))`.
///
/// See also <https://www.w3.org/TR/WGSL/#alignment-and-size> and <https://www.w3.org/TR/WGSL/#address-space-layout-constraints>
#[macro_export]
macro_rules! shader_layout_compat {
    (
        $(#[$attr:meta])*
        $vis:vis struct $struct_name:ident {
            $(
                $(#[$field_attr:meta])*
                $field_vis:vis $field_name:ident : $field_ty:ty
            ),* $(,)?
        }
   ) => {
        $crate::shader_layout!(
            $(#[$attr])*
            $vis struct $struct_name {
                $(
                    $(#[$field_attr])*
                    $field_vis $field_name: $field_ty
                ),*
            }
        );

        $(
            const _ : () = {
                const OFFSET: u64 = core::mem::offset_of!($struct_name, $field_name) as u64;
                const ALIGN_COMPAT: u64 = <$field_ty as $crate::ShaderLayoutCompat>::ALIGN_COMPAT.get();
                assert!(
                    OFFSET.is_multiple_of(ALIGN_COMPAT),
                    concat!(
                        "In a `shader_layout_compat!`, field `",
                        stringify!($struct_name), "::", stringify!($field_name),
                        "` is not properly aligned, with uniform address layout constraints",
                    ),
                );
            };
        )*

        impl $crate::ShaderLayoutCompat for $struct_name {
            const ALIGN_COMPAT: ::core::num::NonZero<u64> = {
                const MEMBER_ALIGNS: &[u64] = &[$(
                    (<$field_ty as $crate::ShaderLayoutCompat>::ALIGN_COMPAT.get())
                ),*];

                let mut max = MEMBER_ALIGNS[0];
                let mut i = 1;
                while i < MEMBER_ALIGNS.len() {
                    if MEMBER_ALIGNS[i] > max {
                        max = MEMBER_ALIGNS[i];
                    }
                    i += 1;
                }
                ::core::num::NonZero::new(max.next_multiple_of(16)).unwrap()
            };
        }
    };
}
