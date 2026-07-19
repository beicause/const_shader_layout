#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg), doc(auto_cfg = false))]
#![no_std]

mod standard;
pub use standard::*;
mod compat;
pub use compat::*;

#[cfg(feature = "glam")]
#[cfg_attr(docsrs, doc(cfg(feature = "glam")))]
mod glam;
#[cfg(feature = "half")]
#[cfg_attr(docsrs, doc(cfg(feature = "half")))]
mod half;
mod primitive;
