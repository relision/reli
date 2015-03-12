// Copyright (c) 2015 by Stacy Prowell.  All rights reserved.
//
// Licensed under the BSD 2-Clause license.  See the file LICENSE
// that is part of this distribution.  This file may not be copied,
// modified, or distributed except according to those terms.extern

//! Linux specific definitions.  This crate contains defintions
//! specific to Linux.

/// Provide Linux specific definitions.
#[allow(dead_code)]
pub mod os_spec {
	use std::env;

	/// Report the target compilation operating system.
	pub fn tos() -> &'static str {
	  "linux"
	}

	/// Locate the configuration files.  This relies on the "XDG Base Directory
	/// Specification" found at:
	/// http://standards.freedesktop.org/basedir-spec/basedir-spec-latest.html
	///
	/// Specifically, Elision stores its configuration and session data in the
	/// first defined and available of the following.
	///
	///   * `$XDG_CONFIG_HOME/elision`
	///   * `$HOME/.config/elision`
	pub fn get_config_dir() -> String {
		match env::var_os("XDG_CONFIG_HOME") {
			Some(value) => {
				let mut result = value;
				result.push("/elision");
				match result.into_string() {
					Ok(rawpath) => { return rawpath; },
					Err(_) => {}
				};
			},
			None => {}
		};
		match env::home_dir() {
			Some(value) => {
				let mut result = value;
				result.push(".config");
				result.push("elision");
				match result.into_os_string().into_string() {
					Ok(rawpath) => { return rawpath; },
					Err(_) => {}
				};
			},
			None => {}
		};
		panic!("Cannot locate configuration files for Elision.  Please set HOME.");
	}
}
