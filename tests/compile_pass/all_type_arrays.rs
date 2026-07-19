use const_shader_layout::shader_layout;
use core::num::{NonZero, Wrapping};
use glam::{
    I16Vec2, I16Vec4, IVec2, IVec4, Mat2, Mat3A, Mat4, Quat, U16Vec2, U16Vec4, UVec2, UVec4, Vec2,
    Vec4,
};
use half::f16;

shader_layout! {
    pub struct AllTypeArrays {
        a1: [f16; 2],
        a2: [i16; 2],
        a3: [u16; 2],
        a4: [NonZero<i16>; 2],
        a5: [NonZero<u16>; 2],
        a6: [Wrapping<i16>; 2],
        a7: [Wrapping<u16>; 2],
        a8: [f32; 2],
        a9: [i32; 2],
        a10: [u32; 2],
        a11: [NonZero<i32>; 2],
        a12: [NonZero<u32>; 2],
        a13: [Wrapping<f32>; 2],
        a14: [Wrapping<i32>; 2],
        a15: [Wrapping<u32>; 2],

        a16: [I16Vec2; 2],
        p1: u32,
        // a17: [I16Vec3; 2],
        a18: [I16Vec4; 2],
        a19: [IVec2; 2],
        // a20: [IVec3; 2],
        p2: [u32; 2],
        a21: [IVec4; 2],
        a22: [Mat2; 2],
        a23: [Mat3A; 2],
        a24: [Mat4; 2],
        a25: [Quat; 2],
        a26: [U16Vec2; 2],
        // a27: [U16Vec3; 2],
        a28: [U16Vec4; 2],
        a29: [UVec2; 2],
        p3: [u32; 2],
        // a30: [UVec3; 2],
        a31: [UVec4; 2],
        a32: [Vec2; 2],
        // a33: [Vec3; 2],
        a34: [Vec4; 2],
    }
}

fn main() {}
