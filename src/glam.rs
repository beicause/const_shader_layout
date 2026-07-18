#[cfg(any(feature = "glam", feature = "glam_32"))]
use glam::{
    I16Vec2, I16Vec3, I16Vec4, IVec2, IVec3, IVec4, Mat2, Mat3A, Mat4, Quat, U16Vec2, U16Vec3,
    U16Vec4, UVec2, UVec3, UVec4, Vec2, Vec3, Vec4,
};
#[cfg(all(feature = "glam_32", not(feature = "glam")))]
use glam_32 as glam;

use crate::{impl_shader_layout, impl_shader_layout_array};

// Vector
impl_shader_layout!(4, 4, I16Vec2, U16Vec2);
impl_shader_layout!(8, 6, I16Vec3, U16Vec3);
impl_shader_layout!(8, 8, I16Vec4, U16Vec4);
impl_shader_layout!(8, 8, IVec2, UVec2, Vec2);
impl_shader_layout!(16, 12, IVec3, UVec3, Vec3);
impl_shader_layout!(16, 16, IVec4, UVec4, Vec4, Quat);

// Matrix
impl_shader_layout!(8, 16, Mat2);
// Can't use `Mat3` as its column vectors are not properly aligned.
impl_shader_layout!(16, 48, Mat3A);
impl_shader_layout!(16, 64, Mat4);

// Array
// Vec3 is not implemented, because total size of `[Vec3; N]` != `N × roundUp(AlignOf(E), SizeOf(E))`
impl_shader_layout_array!(
    I16Vec2, U16Vec2, I16Vec4, U16Vec4, IVec2, UVec2, Vec2, IVec4, UVec4, Vec4, Quat
);
