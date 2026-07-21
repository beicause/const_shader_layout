//@check-pass
use const_shader_layout::{shader_layout, shader_layout_compat};
use core::cmp::Reverse;
use core::num::{NonZero, Saturating, Wrapping};
use glam::{
    I16Vec2, I16Vec3, I16Vec4, IVec2, IVec3, IVec4, Mat2, Mat3A, Mat4, Quat, U16Vec2, U16Vec3,
    U16Vec4, UVec2, UVec3, UVec4, Vec2, Vec3, Vec4,
};
use half::f16;

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
        // 16-bit scalars
        f16_val: f16,
        i16_val: i16,
        u16_val: u16,
        nonzero_i16: NonZero<i16>,
        nonzero_u16: NonZero<u16>,

        // 32-bit scalars
        f32_val: f32,
        i32_val: i32,
        u32_val: u32,
        nonzero_i32: NonZero<i32>,
        nonzero_u32: NonZero<u32>,

        // 16-bit signed vectors
        i16vec2: I16Vec2,
        pad_after_i16vec2: [u16; 2],
        i16vec3: I16Vec3,
        pad_after_i16vec3: [u16; 1],
        i16vec4: I16Vec4,

        // 32-bit signed integer vectors
        ivec2: IVec2,
        ivec3: IVec3,
        pad_before_ivec4: [u32; 1],
        ivec4: IVec4,

        // Matrices & quaternion
        mat2: Mat2,
        mat3a: Mat3A,
        mat4: Mat4,
        quat: Quat,

        // 16-bit unsigned vectors
        u16vec2: U16Vec2,
        pad_after_u16vec2: [u16; 2],
        u16vec3: U16Vec3,
        pad_after_u16vec3: [u16; 1],
        u16vec4: U16Vec4,

        // 32-bit unsigned integer vectors
        uvec2: UVec2,
        uvec3: UVec3,
        pad_before_uvec4: [u32; 1],
        uvec4: UVec4,

        // 32-bit float vectors
        vec2: Vec2,
        pad_before_vec3: [u32; 2],
        vec3: Vec3,
        vec4: Vec4,

        // Nested structs
        nested_std: NestedStd,
        pad_before_nested_compat: [u32; 3],
        nested_compat: NestedCompat,

        // Wrapping
        wrapping_f16: Wrapping<f16>,
        wrapping_i16: Wrapping<i16>,
        wrapping_u16: Wrapping<u16>,
        wrapping_f32: Wrapping<f32>,
        wrapping_i32: Wrapping<i32>,
        wrapping_u32: Wrapping<u32>,

        // Saturating
        saturating_f16: Saturating<f16>,
        saturating_i16: Saturating<i16>,
        saturating_u16: Saturating<u16>,
        saturating_f32: Saturating<f32>,
        saturating_i32: Saturating<i32>,
        saturating_u32: Saturating<u32>,

        // Reverse
        reverse_f16: Reverse<f16>,
        reverse_i16: Reverse<i16>,
        reverse_u16: Reverse<u16>,
        reverse_f32: Reverse<f32>,
        reverse_i32: Reverse<i32>,
        reverse_u32: Reverse<u32>,
    }
}

shader_layout_compat! {
    pub struct AllTypesCompat {
        // 16-bit scalars
        f16_val: f16,
        i16_val: i16,
        u16_val: u16,
        nonzero_i16: NonZero<i16>,
        nonzero_u16: NonZero<u16>,

        // 32-bit scalars
        f32_val: f32,
        i32_val: i32,
        u32_val: u32,
        nonzero_i32: NonZero<i32>,
        nonzero_u32: NonZero<u32>,

        // 16-bit signed vectors
        i16vec2: I16Vec2,
        pad_after_i16vec2: u32,
        i16vec3: I16Vec3,
        pad_after_i16vec3: u16,
        i16vec4: I16Vec4,

        // 32-bit signed integer vectors
        ivec2: IVec2,
        ivec3: IVec3,
        pad_before_ivec4: u32,
        ivec4: IVec4,

        // Matrices & quaternion
        mat2: Mat2,
        mat3a: Mat3A,
        mat4: Mat4,
        quat: Quat,

        // 16-bit unsigned vectors
        u16vec2: U16Vec2,
        pad_after_u16vec2: u32,
        u16vec3: U16Vec3,
        pad_after_u16vec3: u16,
        u16vec4: U16Vec4,

        // 32-bit unsigned integer vectors
        uvec2: UVec2,
        uvec3: UVec3,
        pad_before_uvec4: u32,
        uvec4: UVec4,

        // 32-bit float vectors
        vec2: Vec2,
        pad_before_vec3: Vec2,
        vec3: Vec3,
        vec4: Vec4,

        // Nested compat struct
        nested_compat: NestedCompat,

        // Wrapping
        wrapping_f16: Wrapping<f16>,
        wrapping_i16: Wrapping<i16>,
        wrapping_u16: Wrapping<u16>,
        wrapping_f32: Wrapping<f32>,
        wrapping_i32: Wrapping<i32>,
        wrapping_u32: Wrapping<u32>,

        // Saturating
        saturating_f16: Saturating<f16>,
        saturating_i16: Saturating<i16>,
        saturating_u16: Saturating<u16>,
        saturating_f32: Saturating<f32>,
        saturating_i32: Saturating<i32>,
        saturating_u32: Saturating<u32>,

        // Reverse
        reverse_f16: Reverse<f16>,
        reverse_i16: Reverse<i16>,
        reverse_u16: Reverse<u16>,
        reverse_f32: Reverse<f32>,
        reverse_i32: Reverse<i32>,
        reverse_u32: Reverse<u32>,
    }
}
