//@check-pass
use const_shader_layout::{shader_layout, shader_layout_compat};
use core::num::{NonZero, Saturating, Wrapping};
use glam::{
    I16Vec2, I16Vec3, I16Vec4, IVec2, IVec3, IVec4, Mat2, Mat3A, Mat4, Quat, U16Vec2, U16Vec3,
    U16Vec4, UVec2, UVec3, UVec4, Vec2, Vec3, Vec4,
};

shader_layout! {
    pub struct NestedStd {
        a1: f32,
    }
}

shader_layout_compat! {
    pub struct NestedCompat {
        a1: Vec4,
    }
}

shader_layout! {
    pub struct AllTypes {
        a1: half::f16,
        a2: i16,
        a3: u16,
        a4: NonZero<i16>,
        a5: NonZero<u16>,
        a6: Wrapping<i16>,
        a7: Wrapping<u16>,
        a8: f32,
        a9: i32,
        a10: u32,
        a11: NonZero<i32>,
        a12: NonZero<u32>,
        a13: Wrapping<f32>,
        a14: Wrapping<i32>,
        a15: Wrapping<u32>,

        a16: I16Vec2,
        p1: [u16; 2],
        a17: I16Vec3,
        p2: [u16; 1],
        a18: I16Vec4,
        a19: IVec2,
        a20: IVec3,
        p3: [u32; 1],
        a21: IVec4,
        a22: Mat2,
        a23: Mat3A,
        a24: Mat4,
        a25: Quat,
        a26: U16Vec2,
        p4: [u16; 2],
        a27: U16Vec3,
        p5: [u16; 1],
        a28: U16Vec4,
        a29: UVec2,
        a30: UVec3,
        p6: [u32; 1],
        a31: UVec4,
        a32: Vec2,
        p7: [u32; 2],
        a33: Vec3,
        a34: Vec4,
        a35: NestedStd,
        p8: [u32;3],
        a36: NestedCompat,
        a37: Saturating<i16>,
        a38: Saturating<u16>,
        a39: Saturating<f32>,
        a40: Saturating<i32>,
        a41: Saturating<u32>,
    }
}

shader_layout_compat! {
    pub struct AllTypesCompat {
        a1: half::f16,
        a2: i16,
        a3: u16,
        a4: NonZero<i16>,
        a5: NonZero<u16>,
        a6: Wrapping<i16>,
        a7: Wrapping<u16>,
        a8: f32,
        a9: i32,
        a10: u32,
        a11: NonZero<i32>,
        a12: NonZero<u32>,
        a13: Wrapping<f32>,
        a14: Wrapping<i32>,
        a15: Wrapping<u32>,

        a16: I16Vec2,
        p1: u32,
        a17: I16Vec3,
        p2: u16,
        a18: I16Vec4,
        a19: IVec2,
        a20: IVec3,
        p3: u32,
        a21: IVec4,
        a22: Mat2,
        a23: Mat3A,
        a24: Mat4,
        a25: Quat,
        a26: U16Vec2,
        p4: u32,
        a27: U16Vec3,
        p5: u16,
        a28: U16Vec4,
        a29: UVec2,
        a30: UVec3,
        p6: u32,
        a31: UVec4,
        a32: Vec2,
        p7: Vec2,
        a33: Vec3,
        a34: Vec4,
        a35: NestedCompat,
        a37: Saturating<i16>,
        a38: Saturating<u16>,
        a39: Saturating<f32>,
        a40: Saturating<i32>,
        a41: Saturating<u32>,
    }
}
