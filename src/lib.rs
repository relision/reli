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
#![doc(html_logo_url = "",
		html_favicon_url = "",
		html_root_url = "")]
#![warn(missing_docs)]
#![feature(core)]

fn hold() {
	println!("Holding.");
}