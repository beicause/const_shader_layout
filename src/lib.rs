#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg), doc(auto_cfg = false))]
#![no_std]

#[doc(hidden)]
pub mod internal;

mod standard;
pub use standard::*;
mod compat;
pub use compat::*;

#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
pub use const_shader_layout_derive::{ShaderLayout, ShaderLayoutCompat};

#[cfg(feature = "glam")]
#[cfg_attr(docsrs, doc(cfg(feature = "glam")))]
mod glam;
#[cfg(feature = "half")]
#[cfg_attr(docsrs, doc(cfg(feature = "half")))]
mod half;
mod primitive;
