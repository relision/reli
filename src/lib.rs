//!```text
//!           _ _     _
//!  _ __ ___| (_)___(_) ___  _ __
//! | '__/ _ \ | / __| |/ _ \| '_ \
//! | | |  __/ | \__ \ | (_) | | | |
//! |_|  \___|_|_|___/_|\___/|_| |_|
//! ```
//! The relision term rewriting library.
//!
//! Core library operations.
//!
//! # License
//!
//! Copyright (c) 2015 by Stacy Prowell.  All rights reserved.
//!
//! Licensed under the BSD 2-Clause license.  See the file LICENSE
//! that is part of this distribution.  This file may not be copied,
//! modified, or distributed except according to those terms.

// Tell the documentation system about some icons and require
// documentation.
#![doc(html_logo_url = "https://raw.githubusercontent.com/relision/things/master/graphics/relision.png",
	html_favicon_url = "https://raw.githubusercontent.com/relision/things/master/graphics/favicon.ico",
	html_root_url = "https://github.com/relision")]
#![warn(missing_docs)]

#[macro_use]
extern crate lazy_static;
extern crate num;
mod platform;
mod repl;

/// Get the name of the platform for which this version of relision was
/// compiled.
pub fn get_platform() -> String {
	platform::get_platform().to_string()
}

/// Get the path to the configuration directory where relision should store
/// its persistent state, configuration information, etc.  This should be
/// a readable and writeable folder.
pub fn get_config_dir() -> String {
	platform::get_config_dir().to_string()
}
