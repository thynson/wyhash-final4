//
// This is free and unencumbered software released into the public domain.
//
// Anyone is free to copy, modify, publish, use, compile, sell, or
// distribute this software, either in source code form or as a compiled
// binary, for any purpose, commercial or non-commercial, and by any
// means.
//
// In jurisdictions that recognize copyright laws, the author or authors
// of this software dedicate any and all copyright interest in the
// software to the public domain. We make this dedication for the benefit
// of the public at large and to the detriment of our heirs and
// successors. We intend this dedication to be an overt act of
// relinquishment in perpetuity of all present and future rights to this
// software under copyright law.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
// IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//
// For more information, please refer to <http://unlicense.org/>
//

//!
//! This crate provides a pure rust implementation of wyhash_final4,
//! which is an extremely fast hash function that can be implemented
//! without any machine-specific instructions.
//!
//! The wyhash_final4 is the latest version of wyhash, which is incompatible
//! with previous versions, but resolves the issue of bad seeds.
//!

#![cfg_attr(feature = "no_std", no_std)]

pub mod generics;
mod util;

#[cfg(feature = "wyhash32")]
pub mod wyhash32;

#[cfg(feature = "wyhash32")]
pub use wyhash32::*;

#[cfg(feature = "wyhash32condom")]
pub mod wyhash32condom;

#[cfg(feature = "wyhash32condom")]
pub use wyhash32condom::*;

#[cfg(feature = "wyhash64")]
pub mod wyhash64;

#[cfg(feature = "wyhash64")]
pub use wyhash64::*;

#[cfg(feature = "wyhash64condom")]
pub mod wyhash64condom;

#[cfg(feature = "wyhash64condom")]
pub use wyhash64condom::*;
