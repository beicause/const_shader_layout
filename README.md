# Compile-time checks for whether structs conform to WGSL's [memory layout rules](https://www.w3.org/TR/WGSL/#alignment-and-size).

[![Build](https://github.com/beicause/const_shader_layout/actions/workflows/ci.yml/badge.svg)](https://github.com/beicause/const_shader_layout/actions)
[![License](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue.svg)](https://github.com/beicause/const_shader_layout)
[![Cargo](https://img.shields.io/crates/v/const_shader_layout.svg)](https://crates.io/crates/const_shader_layout)
[![Documentation](https://docs.rs/const_shader_layout/badge.svg)](https://docs.rs/const_shader_layout)


The core of this crate is the [`shader_layout`] macro and the [`ShaderLayout`] trait.
```rust
use const_shader_layout::shader_layout;
use glam::Vec3;

shader_layout! {
    pub struct MyUniform {
        a1: f32,
        a2: [f32; 2],
        a3: [f32; 1],
        a4: Vec3,
        a5: f32
    }
}
```

See <https://github.com/beicause/const_shader_layout/tree/master/tests> for what this supports and checks.

Note: [Uniform address space layout constraints](https://www.w3.org/TR/WGSL/#address-space-layout-constraints) is not checked yet.
