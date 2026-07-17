> [!NOTE]
> This is experimental

Compile-time checks and marker trait `ShaderLayout` for whether structs conform to WGSL's [memory layout rules](https://www.w3.org/TR/WGSL/#alignment-and-size).

Usage:
```rust
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
See https://github.com/beicause/const_shader_layout/tree/master/tests/compile_fail for what this checks.
