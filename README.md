> [!NOTE]
> This is experimental

Compile-time checks and marker trait `ShaderLayout` for whether structs conform to WGSL's [memory layout rules](https://www.w3.org/TR/WGSL/#alignment-and-size).

Usage:
```rust
shader_type! {
    #[derive(Copy, Clone)]
    pub struct MyUniform {
        a1: f32,
        a2: [f32; 2],
        a3: [f32; 1],
        a4: Vec3,
        a5: f32
    }
}
```

Compilation error if not aligned:
```rust
shader_layout! {
    #[derive(Copy, Clone)]
    pub struct MyUniform {
        a1: f32,
        a2: [f32; 2],
        a3: [f32; 2],
        a4: glam::Vec3,
        a5: f32
    }
}
```
```
error[E0080]: evaluation panicked: In a `shader_layout!`, field `MyUniform::a4` is not properly aligned
  --> tests/simple.rs:3:1
   |
 3 | / shader_layout! {
 4 | |     #[derive(Copy, Clone)]
 5 | |     pub struct MyUniform {
 6 | |         a1: f32,
...  |
12 | | }
   | |_^ evaluation of `_` failed here
   |
   = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `shader_layout` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0080]: evaluation panicked: In a `shader_layout!`, struct `MyUniform` size must be a multiple of its align
  --> tests/simple.rs:3:1
   |
 3 | / shader_layout! {
 4 | |     #[derive(Copy, Clone)]
 5 | |     pub struct MyUniform {
 6 | |         a1: f32,
...  |
12 | | }
   | |_^ evaluation of `_` failed here
   |
   = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `shader_layout` (in Nightly builds, run with -Z macro-backtrace for more info)
```

Compilation error if struct size doesn't match shader struct size:
```rust
shader_layout! {
    #[derive(Copy, Clone)]
    pub struct MyUniform {
        a1: f32,
        a2: [f32; 2],
        a3: [f32; 1],
        a4: glam::Vec3,
    }
}
```
```
error[E0080]: evaluation panicked: In a `shader_layout!`, struct `MyUniform` size must be a multiple of its align
  --> tests/simple.rs:3:1
   |
 3 | / shader_layout! {
 4 | |     #[derive(Copy, Clone)]
 5 | |     pub struct MyUniform {
 6 | |         a1: f32,
...  |
11 | | }
   | |_^ evaluation of `_` failed here
   |
   = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `shader_layout` (in Nightly builds, run with -Z macro-backtrace for more info)
```
