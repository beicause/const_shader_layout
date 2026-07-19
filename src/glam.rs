#[cfg(feature = "glam")]
use glam::{
    I16Vec2, I16Vec3, I16Vec4, IVec2, IVec3, IVec4, Mat2, Mat3A, Mat4, Quat, U16Vec2, U16Vec3,
    U16Vec4, UVec2, UVec3, UVec4, Vec2, Vec3, Vec4,
};

use crate::{impl_shader_layout_array_compat, impl_shader_layout_compat};

// Vector
impl_shader_layout_compat!(4, I16Vec2, U16Vec2);
impl_shader_layout_compat!(8, I16Vec3, U16Vec3);
impl_shader_layout_compat!(8, I16Vec4, U16Vec4);
impl_shader_layout_compat!(8, IVec2, UVec2, Vec2);
impl_shader_layout_compat!(16, IVec3, UVec3, Vec3);
impl_shader_layout_compat!(16, IVec4, UVec4, Vec4, Quat);

// Matrix
impl_shader_layout_compat!(8, Mat2);
// Can't use `Mat3` as its column vectors are not properly aligned.
impl_shader_layout_compat!(16, Mat3A);
impl_shader_layout_compat!(16, Mat4);

// Array
// Vec3 should not be implemented, because the size of `[Vec3; N]` != `N × roundUp(AlignOf(E), SizeOf(E))`
impl_shader_layout_array!(I16Vec2, U16Vec2, I16Vec4, U16Vec4, IVec2, UVec2, Vec2);
// Only 16-byte types should implement `ShaderLayoutCompat`
impl_shader_layout_array_compat!(IVec4, UVec4, Vec4, Quat);
impl_shader_layout_array_compat!(Mat2, Mat3A, Mat4);
