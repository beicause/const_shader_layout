> [!NOTE]
> This is experimental

Compile-time checks and marker trait `ShaderAligned` for whether structs conform to shader memory alignment rules.

Usage:
```rust
shader_type! {
    #[derive(bytemuck::NoUninit, Copy, Clone)]
    #[repr(C)]
    pub struct MyUniform {
        a1: f32,
        a2: [f32; 2],
        a3: [f32; 1],
        a4: Vec3,
    }
}
```

Compilation error if not aligned:
```
error[E0080]: evaluation panicked: In a `shader_type!`, field `MyUniform::a4` is not properly aligned
   --> src/lib.rs:94:17
    |
 94 | /                 assert!(
 95 | |                     OFFSET % ALIGN == 0,
 96 | |                     concat!(
 97 | |                         "In a `shader_type!`, field `",
...   |
100 | |                     ),
101 | |                 );
    | |_________________^ evaluation of `_` failed here
...
125 | / shader_type! {
126 | |     #[derive(bytemuck::NoUninit, Copy, Clone)]
127 | |     #[repr(C)]
128 | |     pub struct MyUniform {
...   |
134 | | }
    | |_- in this macro invocation
    |
    = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `shader_type` (in Nightly builds, run with -Z macro-backtrace for more info)
```
