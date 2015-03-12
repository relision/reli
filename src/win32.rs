// Copyright (c) 2015 by Stacy Prowell.  All rights reserved.
//
// Licensed under the BSD 2-Clause license.  See the file LICENSE
// that is part of this distribution.  This file may not be copied,
// modified, or distributed except according to those terms.extern

//! Windows specific definitions.  This crate contains defintions
//! specific to Microsoft Windows.

pub mod os_spec {
	use std::env;

	/// Report the target compilation operating system.
	pub fn tos() -> &'static str {
	  "win32"
	}

	/// Locate the configuration files.  This relies on the
	/// guidance found in the document "Accessing app data with the Windows
	/// Runtime" found at:
	/// https://msdn.microsoft.com/en-us/library/windows/apps/hh464917.aspx.
	///
	/// Only local data is used, stored in application data files.  Environment
	/// variables are used to locate the files.
	///
	/// Specifically, Elision stores its configuration and session data in the
	/// first defined and available of the following.
	///
	///   * `%LOCALAPPDATA%\elision`
	///   * `%USERPROFILE%\AppData\Local\elision`
	///   * `%HOME%\AppData\Local\elision`
	pub fn get_config_dir() -> String {
		match env::var_os("LOCALAPPDATA") {
			Some(value) => {
				match value.into_string() {
					Ok(prefix) => { return prefix + "\\elision"; },
					Err(_) => {}
				};
			},
			None => {}
		};
		match env::var_os("USERPROFILE") {
			Some(value) => {
				match value.into_string() {
					Ok(prefix) => { return prefix + "\\AppData\\Local\\elision"; },
					Err(_) => ()
				};
			},
			None => {}
		};
		match env::home_dir() {
			Some(value) => {
				let mut result = value;
				result.push("AppData");
				result.push("Local");
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
