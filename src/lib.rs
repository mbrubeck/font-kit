// font-kit/src/lib.rs
//
// Copyright © 2018 The Pathfinder Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate euclid;
extern crate lyon_path;
extern crate memmap;

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate log;

#[cfg(target_family = "windows")]
extern crate dwrote;
#[cfg(target_family = "windows")]
extern crate winapi;
#[cfg(target_os = "macos")]
extern crate cocoa;
#[cfg(target_os = "macos")]
extern crate core_foundation;
#[cfg(target_os = "macos")]
extern crate core_graphics;
#[cfg(target_os = "macos")]
extern crate core_text;
#[cfg(not(any(target_os = "macos", target_os = "windows")))]
extern crate freetype;

#[cfg(target_family = "windows")]
#[path = "platform/windows.rs"]
mod platform;
#[cfg(target_os = "macos")]
#[path = "platform/macos.rs"]
mod platform;
#[cfg(not(any(target_os = "macos", target_os = "windows")))]
#[path = "platform/freetype.rs"]
mod platform;

pub mod descriptor;
pub mod family;
pub mod font;
pub mod set;

#[cfg(test)]
pub mod test;
