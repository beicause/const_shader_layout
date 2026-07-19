#[cfg(feature = "glam")]
use glam::{
    I16Vec2, I16Vec3, I16Vec4, IVec2, IVec3, IVec4, Mat2, Mat3A, Mat4, Quat, U16Vec2, U16Vec3,
    U16Vec4, UVec2, UVec3, UVec4, Vec2, Vec3, Vec4,
};

use crate::{impl_shader_layout_compat_array, impl_shader_layout_compat_raw};

// Vector
impl_shader_layout!(4, I16Vec2, U16Vec2);
impl_shader_layout!(8, I16Vec3, U16Vec3);
impl_shader_layout!(8, I16Vec4, U16Vec4);
impl_shader_layout!(8, IVec2, UVec2, Vec2);
impl_shader_layout!(16, IVec3, UVec3, Vec3);
impl_shader_layout!(16, IVec4, UVec4, Vec4, Quat);

// Matrix
impl_shader_layout!(8, Mat2);
// Can't use `Mat3` as its column vectors are not properly aligned.
impl_shader_layout!(16, Mat3A);
impl_shader_layout!(16, Mat4);

// Array
// Vec3 is not implemented, because total size of `[Vec3; N]` != `N × roundUp(AlignOf(E), SizeOf(E))`
impl_shader_layout_array!(
    I16Vec2, U16Vec2, I16Vec4, U16Vec4, IVec2, UVec2, Vec2, IVec4, UVec4, Vec4, Quat
);
impl_shader_layout_array!(Mat2, Mat3A, Mat4);

// Vector
impl_shader_layout_compat_raw!(I16Vec2, U16Vec2);
impl_shader_layout_compat_raw!(I16Vec3, U16Vec3);
impl_shader_layout_compat_raw!(I16Vec4, U16Vec4);
impl_shader_layout_compat_raw!(IVec2, UVec2, Vec2);
impl_shader_layout_compat_raw!(IVec3, UVec3, Vec3);
impl_shader_layout_compat_raw!(IVec4, UVec4, Vec4, Quat);

// Matrix
impl_shader_layout_compat_raw!(Mat2);
// Can't use `Mat3` as its column vectors are not properly aligned.
impl_shader_layout_compat_raw!(Mat3A);
impl_shader_layout_compat_raw!(Mat4);

// Array
impl_shader_layout_compat_array!(IVec4, UVec4, Vec4, Quat);
impl_shader_layout_compat_array!(Mat2, Mat3A, Mat4);
