//
// The MIT License
//
// Copyright 2024 by LAN Xingcan. All Rights Reserved.
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

//!
//! This crate provides a pure rust implementation of wyhash_final4,
//! which is an extremely fast hash function that can be implemented
//! without any machine-specific instructions.
//!
//! The wyhash_final4 is the latest version of wyhash, which is incompatible
//! with previous versions, but resolves the issue of bad seeds that found
//! in previous versions.
//!

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
