/// Marks the type's alignment requirement in shader.
///
/// Note: The `size_of::<Self>` must be equal to its size in shader. Thus [`bool`] should not implement this.
///
/// See also <https://www.w3.org/TR/WGSL/#alignment-and-size>
pub trait ShaderLayout: Clone + Copy + 'static {
    /// The type's alignment requirement in shader.
    const ALIGN: core::num::NonZero<u64> =
        core::num::NonZero::new(align_of::<Self>() as u64).unwrap();
    /// The type's size in shader.
    const SIZE: core::num::NonZero<u64> =
        core::num::NonZero::new(size_of::<Self>() as u64).unwrap();
}

/// Marks the type can be used as array element.
///
/// There is a blanket implementation of `ShaderLayout` for `[T; N]` where `T: ShaderLayoutArrayElement`.
pub trait ShaderLayoutArrayElement: Clone + Copy + 'static {}

/// Implements [`ShaderLayout`] for the types, with the specified alignment.
#[macro_export]
#[doc(hidden)]
macro_rules! impl_shader_layout {
    ($($ty:ty),+$(,)?) => {
        $(impl $crate::ShaderLayout for $ty {})+
    };
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
macro_rules! impl_shader_layout_array_element {
    ($($ty:ty),+$(,)?) => {
        $(
            impl $crate::ShaderLayoutArrayElement for $ty {}

            // Assert array size is equal to `N * roundUp(AlignOf(E), SizeOf(E))`
            const _: () = {
                const ELEMENT_ALIGN: u64 = <$ty as $crate::ShaderLayout>::ALIGN.get();
                const N: usize = 1;
                const ACTUAL_SIZE: u64 = size_of::<[$ty; N]>() as u64;
                const SIZE: u64 = (size_of::<$ty>() as u64).next_multiple_of(ELEMENT_ALIGN) * N as u64;
                if SIZE != ACTUAL_SIZE {
                    let mut msg = $crate::internal::MsgBuf::<256>::new();
                    msg.write_str("Failed to implement `ShaderLayoutArrayElement`: `[")
                        .write_str(stringify!($ty))
                        .write_str("; N]` size (")
                        .write_usize(size_of::<[$ty; N]>())
                        .write_str(" * N) must be equal to its shader size (")
                        .write_u64(SIZE)
                        .write_str(" * N), i.e. the stride must be rounded up to `ALIGN` (")
                        .write_u64(ELEMENT_ALIGN)
                        .write_str(")");
                    panic!("{}", msg.as_str());
                }
            };
        )+
    };
}

#[macro_export]
macro_rules! impl_shader_layout_struct {
       (
           $(#[$attr:meta])*
           $vis:vis struct $struct_name:ident {
               $(
                   $(#[$field_attr:meta])*
                   $field_vis:vis $field_name:ident : $field_ty:ty
               ),* $(,)?
           }
      ) => {
        $(
            const _: () = {
                const OFFSET: u64 = core::mem::offset_of!($struct_name, $field_name) as u64;
                const ALIGN: u64 = <$field_ty as $crate::ShaderLayout>::ALIGN.get();
                if !OFFSET.is_multiple_of(ALIGN) {
                    let mut msg = $crate::internal::MsgBuf::<256>::new();
                    msg.write_str("Failed to implement `ShaderLayout`: field `")
                        .write_str(stringify!($struct_name))
                        .write_str("::")
                        .write_str(stringify!($field_name))
                        .write_str("` (`")
                        .write_str(stringify!($field_ty))
                        .write_str("`) is not properly aligned. The offset is ")
                        .write_u64(OFFSET)
                        .write_str(" but required align is ")
                        .write_u64(ALIGN);
                    panic!("{}", msg.as_str());
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
                let mut msg = $crate::internal::MsgBuf::<256>::new();
                msg.write_str("Failed to implement `ShaderLayout`: struct `")
                    .write_str(stringify!($struct_name))
                    .write_str("` size (")
                    .write_usize(size_of::<$struct_name>())
                    .write_str(") must be equal to its shader size (")
                    .write_u64(SIZE)
                    .write_str("), i.e. rounded up to its `ALIGN` (")
                    .write_u64(<$struct_name as $crate::ShaderLayout>::ALIGN.get())
                    .write_str(")");
                panic!("{}", msg.as_str());
            }
        };

        impl $crate::ShaderLayoutArrayElement for $struct_name {}
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

        $crate::impl_shader_layout_struct!{
            $(#[$attr])*
            $vis struct $struct_name {
                $(
                    $(#[$field_attr])*
                    $field_vis $field_name: $field_ty
                ),*
            }
        }
    };
}
