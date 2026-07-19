use crate::ShaderLayout;

/// Marks the type's uniform-compatible alignment requirement in shader, i.e. with uniform address layout constraints.
///
/// If the type is constrained, its size in shader will be equal to [`ShaderLayoutCompat::SIZE_COMPAT`] Instead of `size_of::<Self>`.
///
/// See also <https://www.w3.org/TR/WGSL/#alignment-and-size> and <https://www.w3.org/TR/WGSL/#address-space-layout-constraints>
pub trait ShaderLayoutCompat: ShaderLayout {
    /// The type's alignment requirement with uniform address layout constraints in shader.
    /// If the type is not constrained, it should be [`ShaderLayout::ALIGN`].
    const ALIGN_COMPAT: core::num::NonZero<u64> = Self::ALIGN;
    /// The type's size requirement with uniform address layout constraints in shader.
    /// If the type is not constrained, it should be `size_of::<Self>()`.
    const SIZE_COMPAT: core::num::NonZero<u64> =
        core::num::NonZero::new(size_of::<Self>() as u64).unwrap();
}

/// Implements [`ShaderLayoutCompat`] (also implements [`ShaderLayout`]) for the primitive types, with their original alignment.
#[macro_export]
#[doc(hidden)]
macro_rules! impl_shader_layout_compat_primitive {
    ($($ty:ty),+$(,)?) => {
        $(
            $crate::impl_shader_layout_primitive!($ty);
            impl $crate::ShaderLayoutCompat for $ty {}
        )+
    };
}

/// Implements [`ShaderLayoutCompat`] (also implements [`ShaderLayout`]) for the types, with the specified alignment.
#[macro_export]
#[doc(hidden)]
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
    ($align:expr, $align_compat:expr, $size_compat:expr $(, $ty:ty)+$(,)?) => {
        const _: () ={
            assert!((($size_compat) as u64).is_multiple_of(16u64));
        };
        $(
            $crate::impl_shader_layout!($align, $ty);
            impl $crate::ShaderLayoutCompat for $ty {
                const ALIGN_COMPAT: ::core::num::NonZero<u64> = ::core::num::NonZero::new($align_compat).unwrap();
                const SIZE_COMPAT: ::core::num::NonZero<u64> = ::core::num::NonZero::new($size_compat).unwrap();
            }
        )+
    };
}

/// Implements [`ShaderLayoutCompat`] (also implements [`ShaderLayout`]) for `[T; N]` for types implemented [`ShaderLayoutCompat`].
///
/// Different from [`ShaderLayout`], the stride of array must be a multiple of 16.
///
/// Checks at compile-time:
/// * Array size must be equal to `N * roundUp(16, roundUp(AlignOf(E), SizeOf(E)))`.
///
/// See also <https://www.w3.org/TR/WGSL/#alignment-and-size> and <https://www.w3.org/TR/WGSL/#address-space-layout-constraints>
#[macro_export]
#[doc(hidden)]
macro_rules! impl_shader_layout_array_compat {
    ($($ty:ty),+$(,)?) => {
        $(
            $crate::impl_shader_layout_array!($ty);
            impl<const N: usize> $crate::ShaderLayoutCompat for [$ty; N]
            {
                const ALIGN_COMPAT: ::core::num::NonZero<u64> = ::core::num::NonZero::new(
                    <$ty as $crate::ShaderLayout>::ALIGN.get().next_multiple_of(16)
                ).unwrap();
                const SIZE_COMPAT: ::core::num::NonZero<u64> = ::core::num::NonZero::new(
                    (size_of::<$ty>() as u64).next_multiple_of(<$ty as $crate::ShaderLayout>::ALIGN.get()).next_multiple_of(16) * N as u64
                ).unwrap();
            }

            // Assert array size is equal to `N * roundUp(16, roundUp(AlignOf(E), SizeOf(E)))`
            const _: () = {
                const N: usize = 1;
                const SIZE: u64 = <[$ty; N] as $crate::ShaderLayoutCompat>::SIZE_COMPAT.get();
                const_format::assertcp!(
                    SIZE == size_of::<[$ty; N]>() as u64,
                        "`[{}; N]` size ({} * N) must be equal to its shader size ({} * N), i.e. `N * roundUp(16, roundUp(AlignOf(E), SizeOf(E)))`",
                        stringify!($ty),
                        size_of::<[$ty; N]>(),
                        SIZE,
                );
            };
        )+
    };
}

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
            const _: () = {
                const MEMBER_ALIGN_COMPAT: u64 = <$field_ty as $crate::ShaderLayoutCompat>::ALIGN_COMPAT.get();
                const MEMBER_SIZE_COMPAT: u64 = <$field_ty as $crate::ShaderLayoutCompat>::SIZE_COMPAT.get();
                const MEMBER_SIZE: u64 = size_of::<$field_ty>() as u64;
                const MEMBER_OFFSET: u64 = core::mem::offset_of!($struct_name, $field_name) as u64;

                const_format::assertcp!(
                    MEMBER_SIZE == MEMBER_SIZE_COMPAT,
                        "Failed to impl `ShaderLayoutCompat`: Field `{}::{}` size ({}) must be {} due to uniform layout constraints",
                        stringify!($struct_name),
                        stringify!($field_name),
                        MEMBER_SIZE,
                        MEMBER_SIZE_COMPAT,
                );

                const_format::assertcp!(
                    MEMBER_OFFSET.is_multiple_of(MEMBER_ALIGN_COMPAT),
                        "Failed to impl `ShaderLayoutCompat`: Field `{}::{}` is not properly aligned. \
                        The offset is {} but required align is {}",
                        stringify!($struct_name),
                        stringify!($field_name),
                        MEMBER_OFFSET,
                        MEMBER_ALIGN_COMPAT,
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
            const SIZE_COMPAT: ::core::num::NonZero<u64> = {
                const MEMBER_SIZES: &[u64] = &[$(
                    (<$field_ty as $crate::ShaderLayoutCompat>::SIZE_COMPAT.get())
                ),*];

                let mut sum = MEMBER_SIZES[0];
                let mut i = 1;
                while i < MEMBER_SIZES.len() {
                    sum += MEMBER_SIZES[i];
                    i += 1;
                }
                ::core::num::NonZero::new(sum.next_multiple_of(16)).unwrap()
            };
        }
    };
}
