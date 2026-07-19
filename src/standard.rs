/// Marks the type's alignment requirement in shader.
///
/// Note: The `size_of::<Self>` must be equal to its size in shader. Thus [`bool`] should not implement this.
///
/// See also <https://www.w3.org/TR/WGSL/#alignment-and-size>
pub trait ShaderLayout: Clone + Copy + 'static {
    /// The type's alignment requirement in shader.
    const ALIGN: core::num::NonZero<u64>;
}

/// Implements [`ShaderLayout`] for the primitive types, with their original alignment.
#[macro_export]
#[doc(hidden)]
macro_rules! impl_shader_layout_primitive {
    ($($ty:ty),+$(,)?) => {
        $(impl $crate::ShaderLayout for $ty {
            const ALIGN: ::core::num::NonZero<u64> = ::core::num::NonZero::new(align_of::<$ty>() as u64).unwrap();
        })+
    };
}

/// Implements [`ShaderLayout`] for the types, with the specified alignment.
#[macro_export]
#[doc(hidden)]
macro_rules! impl_shader_layout {
    ($align:expr $(, $ty:ty)+$(,)?) => {
        $(
            impl $crate::ShaderLayout for $ty {
                const ALIGN: ::core::num::NonZero<u64> = ::core::num::NonZero::new($align).unwrap();
            }
        )+
    };
}

/// Implements [`ShaderLayout`] for `[T; N]` for types implemented [`ShaderLayout`].
///
/// Checks at compile-time:
/// * Array size must be equal to `N * roundUp(AlignOf(E), SizeOf(E))`.
///
/// See also <https://www.w3.org/TR/WGSL/#alignment-and-size>
#[macro_export]
#[doc(hidden)]
macro_rules! impl_shader_layout_array {
    ($($ty:ty),+$(,)?) => {
        $(
            impl<const N: usize> $crate::ShaderLayout for [$ty; N]
            {
                const ALIGN: ::core::num::NonZero<u64> = <$ty as $crate::ShaderLayout>::ALIGN;
            }

            // Assert array size is equal to `N * roundUp(AlignOf(E), SizeOf(E))`
            const _: () = {
                const N: usize = 1;
                const SIZE: u64 = (size_of::<$ty>() as u64).next_multiple_of(<$ty as $crate::ShaderLayout>::ALIGN.get()) * N as u64;
                const_format::assertcp!(
                    SIZE == size_of::<[$ty; N]>() as u64,
                        "`[{}; N]` size ({} * N) must be equal to its shader size ({} * N), i.e. `N * roundUp(AlignOf(E), SizeOf(E))`",
                        stringify!($ty),
                        size_of::<[$ty; N]>(),
                        SIZE,
                );
            };
        )+
    };
}

/// Checks if all the struct's fields conform to shader layout then implements [`ShaderLayout`] for this struct, or fails at compile-time.
///
/// Checks at compile-time:
/// * For each field, `core::mem::offset_of!(struct, field)` must be equal to its [`ShaderLayout::ALIGN`].
/// * Struct size must be equal to `roundUp(AlignOf(S), SizeOf(S)))`.
///
/// See also <https://www.w3.org/TR/WGSL/#alignment-and-size>
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
            const _: () = {
                const OFFSET: u64 = core::mem::offset_of!($struct_name, $field_name) as u64;
                const ALIGN: u64 = <$field_ty as $crate::ShaderLayout>::ALIGN.get();
                const_format::assertcp!(
                    OFFSET.is_multiple_of(ALIGN),
                        "When implementing `ShaderLayout`, field `{}::{}` is not properly aligned. The offset is {} but required align is {}",
                        stringify!($struct_name),
                        stringify!($field_name),
                        OFFSET,
                        ALIGN,
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
        const _: () = {
            const SIZE: u64 = (size_of::<$struct_name>() as u64).next_multiple_of(<$struct_name as $crate::ShaderLayout>::ALIGN.get());
            const_format::assertcp!(
                (size_of::<$struct_name>() as u64) == SIZE,
                "When implementing `ShaderLayout`, struct `{}` size ({}) must be equal to its shader size ({}), i.e. `roundUp(AlignOf(S), SizeOf(S)))`",
                stringify!($struct_name),
                size_of::<$struct_name>(),
                SIZE,
            );
        };
    };
}
