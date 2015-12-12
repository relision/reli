//! Core library operations.
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

#[macro_use]
extern crate lazy_static;
extern crate num;
pub mod platform;
pub mod repl;
