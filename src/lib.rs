//! The relision library.
//!
//! ```text
//!           _ _     _
//!  _ __ ___| (_)___(_) ___  _ __
//! | '__/ _ \ | / __| |/ _ \| '_ \
//! | | |  __/ | \__ \ | (_) | | | |
//! |_|  \___|_|_|___/_|\___/|_| |_|
//! ```
//! The relision term rewriting library.
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

// Use a crate for defining single-initialization complex static data.
#[macro_use]
extern crate lazy_static;

// Use a crate for arbitrary precision integers.
#[macro_use]
extern crate num;

// Modules (namespaces) provided by this library are all defined here.
pub mod prelude;		// Common definitions for programs that want to use relision.
pub mod platform;		// Platform-specific functions and definitions.
pub mod repl;			// Implementation of the REPL for relision.
pub mod terms;			// Implementation of terms.
