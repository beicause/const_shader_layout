//@check-pass
use const_shader_layout::{ShaderLayoutCompatArrayElement, shader_layout, shader_layout_compat};
use core::cmp::Reverse;
use core::num::{NonZero, Saturating, Wrapping};
use glam::{
    I16Vec2, I16Vec4, IVec2, IVec4, Mat2, Mat3A, Mat4, Quat, U16Vec2, U16Vec4, UVec2, UVec4, Vec2,
    Vec3A, Vec4,
};
use half::f16;

shader_layout! {
    struct NestedStd {
        a1: f32,
    }
}

shader_layout_compat! {
    pub(crate) struct NestedCompat {
        a1: Vec4,
    }
}
impl ShaderLayoutCompatArrayElement for NestedCompat {}

shader_layout! {
    pub struct AllTypeArrays {
        // 16-bit scalars
        pub f16_arr: [f16; 2],
        pub(crate) i16_arr: [i16; 2],
        u16_arr: [u16; 2],
        nonzero_i16_arr: [NonZero<i16>; 2],
        nonzero_u16_arr: [NonZero<u16>; 2],

        // 32-bit scalars
        f32_arr: [f32; 2],
        i32_arr: [i32; 2],
        u32_arr: [u32; 2],
        nonzero_i32_arr: [NonZero<i32>; 2],
        nonzero_u32_arr: [NonZero<u32>; 2],

        // 16-bit signed vectors
        i16vec2_arr: [I16Vec2; 2],
        pad_after_i16vec2: u32,
        // i16vec3_arr: [I16Vec3; 2],
        i16vec4_arr: [I16Vec4; 2],

        // 32-bit signed integer vectors
        ivec2_arr: [IVec2; 2],
        // ivec3_arr: [IVec3; 2],
        pad_before_ivec4: [u32; 2],
        ivec4_arr: [IVec4; 2],

        // Matrices & quaternion
        mat2_arr: [Mat2; 2],
        mat3a_arr: [Mat3A; 2],
        mat4_arr: [Mat4; 2],
        quat_arr: [Quat; 2],

        // 16-bit unsigned vectors
        u16vec2_arr: [U16Vec2; 2],
        // u16vec3_arr: [U16Vec3; 2],
        u16vec4_arr: [U16Vec4; 2],

        // 32-bit unsigned integer vectors
        uvec2_arr: [UVec2; 2],
        // uvec3_arr: [UVec3; 2],
        pad_before_uvec4: [u32; 2],
        uvec4_arr: [UVec4; 2],

        // 32-bit float vectors
        vec2_arr: [Vec2; 2],
        // vec3_arr: [Vec3; 2],
        vec3a_arr: [Vec3A; 2],
        vec4_arr: [Vec4; 2],

        // Nested structs
        nested_std_arr: [NestedStd; 2],
        pad_before_nested_compat: [u32; 2],
        nested_compat_arr: [NestedCompat; 2],

        // Wrapping
        wrapping_f16_arr: [Wrapping<f16>; 2],
        wrapping_i16_arr: [Wrapping<i16>; 2],
        wrapping_u16_arr: [Wrapping<u16>; 2],
        wrapping_f32_arr: [Wrapping<f32>; 2],
        wrapping_i32_arr: [Wrapping<i32>; 2],
        wrapping_u32_arr: [Wrapping<u32>; 2],

        // Saturating
        saturating_f16_arr: [Saturating<f16>; 2],
        saturating_i16_arr: [Saturating<i16>; 2],
        saturating_u16_arr: [Saturating<u16>; 2],
        saturating_f32_arr: [Saturating<f32>; 2],
        saturating_i32_arr: [Saturating<i32>; 2],
        saturating_u32_arr: [Saturating<u32>; 2],

        // Reverse
        reverse_f16_arr: [Reverse<f16>; 2],
        reverse_i16_arr: [Reverse<i16>; 2],
        reverse_u16_arr: [Reverse<u16>; 2],
        reverse_f32_arr: [Reverse<f32>; 2],
        reverse_i32_arr: [Reverse<i32>; 2],
        reverse_u32_arr: [Reverse<u32>; 2],
    }
}

shader_layout_compat! {
    pub struct AllTypeArraysCompat {

        // Matrices & quaternion
        pub mat2_arr: [Mat2; 2],
        pub(crate) mat3a_arr: [Mat3A; 2],
        mat4_arr: [Mat4; 2],
        quat_arr: [Quat; 2],

        // vectors
        ivec4_arr: [IVec4; 2],
        uvec4_arr: [UVec4; 2],
        vec3a_arr: [Vec3A; 2],
        vec4_arr: [Vec4; 2],

        // Nested compat struct
        nested_compat_arr: [NestedCompat; 2],
    }
}
