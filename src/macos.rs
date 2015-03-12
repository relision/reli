// Copyright (c) 2015 by Stacy Prowell.  All rights reserved.
//
// Licensed under the BSD 2-Clause license.  See the file LICENSE
// that is part of this distribution.  This file may not be copied,
// modified, or distributed except according to those terms.extern

//! Mac OS X specific definitions.  This crate contains defintions
//! specific to Apple Mac OS X.

pub mod os_spec {
	use std::env;

	/// Report the target compilation operating system.
	pub fn tos() -> &'static str {
	  "macos"
	}

	/// Locate the configuration files.  This relies on the "Mac App Programming
	/// Guide" found at:
	/// https://developer.apple.com/library/mac/documentation/General/Conceptual/MOSXAppProgrammingGuide/AppRuntime/AppRuntime.html
	///
	/// Specifically, Elision stores its configuration and session data in the
	/// `$HOME/Application Support/elision` folder.
	pub fn get_config_dir() -> String {
		match env::home_dir() {
			Some(value) => {
				let mut result = value;
				result.push("Library");
				result.push("Application Support");
				result.push("elision");
				match result.into_os_string().into_string() {
					Ok(res) => { return res; },
					Err(_) => {}
				};
			},
			None => {}
		};
		panic!("Cannot locate configuration files for Elision.  Please set HOME.");
	}
}
