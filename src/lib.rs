// Copyright (c) 2015 by Stacy Prowell.  All rights reserved.
//
// Licensed under the BSD 2-Clause license.  See the file LICENSE
// that is part of this distribution.  This file may not be copied,
// modified, or distributed except according to those terms.extern

//! The Elision term rewriting library.
//!
//! Elision is a custom term rewriter intended for use in analyzing
//! and computing the behavior of compiled software.  Elision is
//! Turing complete.

// Tell the documentation system about some icons and require
// documentation.  Enable core.
#![doc(html_logo_url = "https://raw.githubusercontent.com/relision/things/master/graphics/relision.png",
		html_favicon_url = "https://raw.githubusercontent.com/relision/things/master/graphics/favicon.ico",
		html_root_url = "https://github.com/relision")]
#![warn(missing_docs)]

/* Load platform specific definitions. */

mod win32;
mod linux;
mod macos;

#[cfg(target_os = "linux")]
pub use linux::os_spec;
#[cfg(target_os = "win32")]
pub use win32::os_spec;
#[cfg(target_os = "macos")]
pub use macos::os_spec;
#[cfg(not(any(target_os = "macos",
              target_os = "win32",
              target_os = "linux")))]
pub use linux::os_spec;

use std::fs::File;
use std::path::Path;

/// Get the configuration folder.
pub fn get_config_dir() -> &'static Path {
	let path = Path::new(os_spec::get_config_dir());
	std::old_io::fs::mkdir_recursive(&path);
	return path;
}