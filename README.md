# Compile-time checks for whether structs conform to WGSL's [memory layout rules](https://www.w3.org/TR/WGSL/#alignment-and-size).

[![Build](https://github.com/beicause/const_shader_layout/actions/workflows/ci.yml/badge.svg)](https://github.com/beicause/const_shader_layout/actions)
[![License](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue.svg)](https://github.com/beicause/const_shader_layout)
[![Cargo](https://img.shields.io/crates/v/const_shader_layout.svg)](https://crates.io/crates/const_shader_layout)
[![Documentation](https://docs.rs/const_shader_layout/badge.svg)](https://docs.rs/const_shader_layout)


The core of this crate is the [`shader_layout`], [`shader_layout_compat`] macros and the [`ShaderLayout`], [`ShaderLayoutCompat`] traits.

`ShaderLayout` is corresponding to <https://www.w3.org/TR/WGSL/#alignment-and-size>.

`ShaderLayoutCompat` is a subset of `ShaderLayout` and is corresponding to [uniform address space layout constraints](https://www.w3.org/TR/WGSL/#address-space-layout-constraints) without [uniform_buffer_standard_layout](https://www.w3.org/TR/WGSL/#language_extension-uniform_buffer_standard_layout).

```rust
use const_shader_layout::{shader_layout, shader_layout_compat, ShaderLayout, ShaderLayoutCompat};
use glam::{Vec3, Vec4};

shader_layout! {
    pub struct MyStorage {
        a1: f32,
        a2: [f32; 2],
        a3: [f32; 1],
        a4: Vec3,
        a5: f32
    }
}
const {
    assert!(<MyStorage as ShaderLayout>::ALIGN.get() == 16);
}

shader_layout_compat! {
    pub struct MyUniform {
        a1: [Vec4; 2],
        a3: Vec3,
        a4: f32
    }
}
const {
    assert!(<MyUniform as ShaderLayoutCompat>::ALIGN_COMPAT.get() == 16);
}
```

See <https://github.com/beicause/const_shader_layout/tree/master/tests> for what this supports and checks.

This doesn't provide any byte conversion methods. Instead, it's intended to be used with other libraries such as [bytemuck](https://docs.rs/bytemuck/latest/bytemuck/index.html).
