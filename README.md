# Compile-time checks to ensure structs conform to WGSL's [memory layout rules](https://www.w3.org/TR/WGSL/#alignment-and-size)

[![Build](https://github.com/beicause/const_shader_layout/actions/workflows/ci.yml/badge.svg)](https://github.com/beicause/const_shader_layout/actions)
[![License](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue.svg)](https://github.com/beicause/const_shader_layout)
[![Cargo](https://img.shields.io/crates/v/const_shader_layout.svg)](https://crates.io/crates/const_shader_layout)
[![Documentation](https://docs.rs/const_shader_layout/badge.svg)](https://docs.rs/const_shader_layout)

The core of this crate is the [`ShaderLayout`], [`ShaderLayoutCompat`] derive proc-macros
(or declarative macros if `derive` feature is disabled), and the [`ShaderLayout`], [`ShaderLayoutCompat`] traits.

`ShaderLayout` corresponds to <https://www.w3.org/TR/WGSL/#alignment-and-size>.

`ShaderLayoutCompat` is a stricter subset that enforces the
[uniform address space layout constraints](https://www.w3.org/TR/WGSL/#address-space-layout-constraints)
(without the `uniform_buffer_standard_layout` extension). Every type implementing `ShaderLayoutCompat`
also implements `ShaderLayout`.

Both macros validate every field's alignment and the overall struct size at compile time. If a constraint
is violated, compilation fails with a clear error message:

```text
shader_layout! {
    pub struct OffsetUnaligned {
        a1: f32,        // align 4
        a4: glam::Vec3, // align 16 — offset 4 is not a multiple of 16!
    }
}
// error[E0080]: evaluation panicked: Failed to implement `ShaderLayout`: field `OffsetUnaligned::a4` (`glam::Vec3`) is not properly aligned. The offset is 4 but required align is 16
```

```rust
use const_shader_layout::{shader_layout, shader_layout_compat, ShaderLayout, ShaderLayoutCompat};
use glam::{Vec2, Vec3, Vec4};

// Use derive proc-macros
#[derive(Clone, Copy, ShaderLayout)]
#[repr(C)]
pub struct MyStorage {
    a1: f32,
    a2: [f32; 2],
    a3: [f32; 1],
    a4: Vec3,
    a5: f32,
    a6: Vec3,
    p1: f32, // Padding needed otherwise struct size (44) won't match shader size (48).
}

// Or use declarative macros
shader_layout_compat! {
    pub struct Nested {
        a1: [Vec4; 2],
        a2: Vec3,
        a3: f32
    }
}

shader_layout_compat! {
    pub struct MyUniform {
        a1: Nested,
        a2: Vec3,
        // Padding is implicit, because struct size is 64 aligned to 16 in `repr(C)` which matches shader size (64).
    }
}
```

See <https://github.com/beicause/const_shader_layout/tree/master/tests> for what this supports and checks.

## Features

| Feature | Description |
|---------|-------------|
| `derive`(default) | Enable `ShaderLayout`, `ShaderLayoutCompat` derive macros  |
| `glam` (default) | Implements `ShaderLayout`/`ShaderLayoutCompat` for `glam` types |
| `half` | Implements `ShaderLayout`/`ShaderLayoutCompat` for `half::f16` and array of it |
| `std` (default), `libm`, `nostd-libm` | Re-export `glam`'s corresponding features |

This crate focuses on layout validation only. For safe cast/transmute utilities, pair it with other crates like [bytemuck](https://docs.rs/bytemuck/).
