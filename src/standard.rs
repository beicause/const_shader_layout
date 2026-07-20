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

/// Implements [`ShaderLayout`] for a custom array type for elements implemented [`ShaderLayout`].
///
/// Checks at compile-time:
/// * Array size must be equal to `N * roundUp(AlignOf(E), SizeOf(E))`.
///
/// See also <https://www.w3.org/TR/WGSL/#alignment-and-size>
#[macro_export]
#[doc(hidden)]
macro_rules! impl_shader_layout_custom_array {
    ($elem_ty:ty, $array_ty:ty, $n:expr) => {
        impl $crate::ShaderLayout for $array_ty {
            const ALIGN: ::core::num::NonZero<u64> = <$elem_ty as $crate::ShaderLayout>::ALIGN;
        }

        // Assert array size is equal to `N * roundUp(AlignOf(E), SizeOf(E))`
        const _: () = {
            const N: usize = $n;
            const SIZE: u64 = (size_of::<$elem_ty>() as u64)
                .next_multiple_of(<$elem_ty as $crate::ShaderLayout>::ALIGN.get())
                * N as u64;
            if SIZE != size_of::<$array_ty>() as u64 {
                let mut buf = [0u8; 256];
                let mut pos = 0usize;
                pos = $crate::internal::write_str(
                    &mut buf,
                    pos,
                    "Failed to implement `ShaderLayout`: array `",
                );
                pos = $crate::internal::write_str(&mut buf, pos, stringify!($array_ty));
                pos = $crate::internal::write_str(&mut buf, pos, "` size (");
                pos = $crate::internal::write_usize(&mut buf, pos, size_of::<$array_ty>());
                pos = $crate::internal::write_str(
                    &mut buf,
                    pos,
                    ") must be equal to its shader size (",
                );
                pos = $crate::internal::write_u64(&mut buf, pos, SIZE);
                pos = $crate::internal::write_str(
                    &mut buf,
                    pos,
                    "), i.e. the stride must be rounded up to `ALIGN` (",
                );
                pos = $crate::internal::write_u64(
                    &mut buf,
                    pos,
                    <$elem_ty as $crate::ShaderLayout>::ALIGN.get(),
                );
                pos = $crate::internal::write_str(&mut buf, pos, ")");
                let msg = $crate::internal::buf_to_str(&buf, pos);
                panic!("{}", msg);
            }
        };
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
                const ELEMENT_ALIGN: u64 = <$ty as $crate::ShaderLayout>::ALIGN.get();
                const N: usize = 1;
                const ACTUAL_SIZE: u64 = size_of::<[$ty; N]>() as u64;
                const SIZE: u64 = (size_of::<$ty>() as u64).next_multiple_of(ELEMENT_ALIGN) * N as u64;
                if SIZE != ACTUAL_SIZE {
                    let mut buf = [0u8; 256];
                    let mut pos = 0usize;
                    pos = $crate::internal::write_str(&mut buf, pos, "`[");
                    pos = $crate::internal::write_str(&mut buf, pos, stringify!($ty));
                    pos = $crate::internal::write_str(&mut buf, pos, "; N]` size (");
                    pos = $crate::internal::write_usize(&mut buf, pos, size_of::<[$ty; N]>());
                    pos = $crate::internal::write_str(&mut buf, pos, " * N) must be equal to its shader size (");
                    pos = $crate::internal::write_u64(&mut buf, pos, SIZE);
                    pos = $crate::internal::write_str(&mut buf, pos, " * N), i.e. the stride must be rounded up to `ALIGN` (");
                    pos = $crate::internal::write_u64(&mut buf, pos, ELEMENT_ALIGN);
                    pos = $crate::internal::write_str(&mut buf, pos, ")");
                    let msg = $crate::internal::buf_to_str(&buf, pos);
                    panic!("{}", msg);
                }
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
                if !OFFSET.is_multiple_of(ALIGN) {
                    let mut buf = [0u8; 256];
                    let mut pos = 0usize;
                    pos = $crate::internal::write_str(&mut buf, pos, "Failed to implement `ShaderLayout`: field `");
                    pos = $crate::internal::write_str(&mut buf, pos, stringify!($struct_name));
                    pos = $crate::internal::write_str(&mut buf, pos, "::");
                    pos = $crate::internal::write_str(&mut buf, pos, stringify!($field_name));
                    pos = $crate::internal::write_str(&mut buf, pos, "` (`");
                    pos = $crate::internal::write_str(&mut buf, pos, stringify!($field_ty));
                    pos = $crate::internal::write_str(&mut buf, pos, "`) is not properly aligned. The offset is ");
                    pos = $crate::internal::write_u64(&mut buf, pos, OFFSET);
                    pos = $crate::internal::write_str(&mut buf, pos, " but required align is ");
                    pos = $crate::internal::write_u64(&mut buf, pos, ALIGN);
                    let msg = $crate::internal::buf_to_str(&buf, pos);
                    panic!("{}", msg);
                }
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
            if size_of::<$struct_name>() as u64 != SIZE {
                let mut buf = [0u8; 256];
                let mut pos = 0usize;
                pos = $crate::internal::write_str(&mut buf, pos, "Failed to implement `ShaderLayout`: struct `");
                pos = $crate::internal::write_str(&mut buf, pos, stringify!($struct_name));
                pos = $crate::internal::write_str(&mut buf, pos, "` size (");
                pos = $crate::internal::write_usize(&mut buf, pos, size_of::<$struct_name>());
                pos = $crate::internal::write_str(&mut buf, pos, ") must be equal to its shader size (");
                pos = $crate::internal::write_u64(&mut buf, pos, SIZE);
                pos = $crate::internal::write_str(&mut buf, pos, "), i.e. rounded up to its `ALIGN` (");
                pos = $crate::internal::write_u64(&mut buf, pos, <$struct_name as $crate::ShaderLayout>::ALIGN.get());
                pos = $crate::internal::write_str(&mut buf, pos, ")");
                let msg = $crate::internal::buf_to_str(&buf, pos);
                panic!("{}", msg);
            }
        };
    };
}
