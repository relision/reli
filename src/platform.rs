//! ```text
//!           _ _     _
//!  _ __ ___| (_)___(_) ___  _ __
//! | '__/ _ \ | / __| |/ _ \| '_ \
//! | | |  __/ | \__ \ | (_) | | | |
//! |_|  \___|_|_|___/_|\___/|_| |_|
//! ```
//! The relision term rewriting library.
//!
//! Provide all platform-specific definitions.
//!
//! # License
//!
//! Copyright (c) 2015 by Stacy Prowell.  All rights reserved.
//!
//! Licensed under the BSD 2-Clause license.  See the file LICENSE
//! that is part of this distribution.  This file may not be copied,
//! modified, or distributed except according to those terms.

use std::env;

/// Get the name of the platform for which this version of relision was
/// compiled.
pub fn get_platform() -> &'static str {
    if cfg!(target_os = "macos") {
        "macos"
    } else if cfg!(target_os = "linux") {
        "linux"
    } else if cfg!(target_os = "windows") {
        "windows"
    } else {
        "unspecified"
    }
}

/// Get the path to the configuration directory where relision should store
/// its persistent state, configuration information, etc.  This should be
/// a readable and writeable folder.
///
/// # Mac OS X
/// Locate the configuration files.  This relies on the "Mac App Programming
/// Guide" found at:
/// https://developer.apple.com/library/mac/documentation/General/Conceptual/MOSXAppProgrammingGuide/AppRuntime/AppRuntime.html
///
/// Specifically, relision stores its configuration and session data in the
/// `$HOME/Application Support/relision` folder.
///
/// # Windows
/// Locate the configuration files.  This relies on the
/// guidance found in the document "Accessing app data with the Windows
/// Runtime" found at:
/// https://msdn.microsoft.com/en-us/library/windows/apps/hh464917.aspx.
///
/// Only local data is used, stored in application data files.  Environment
/// variables are used to locate the files.
///
/// Specifically, relision stores its configuration and session data in the
/// first defined and available of the following.
///
///   * `%LOCALAPPDATA%\relision`
///   * `%USERPROFILE%\AppData\Local\relision`
///   * `%HOME%\AppData\Local\relision`
///
/// # Default Behavior Including Linux and Unix
/// Locate the configuration files.  This relies on the "XDG Base Directory
/// Specification" found at:
/// http://standards.freedesktop.org/basedir-spec/basedir-spec-latest.html
///
/// Specifically, relision stores its configuration and session data in the
/// first defined and available of the following.
///
///   * `$XDG_CONFIG_HOME/relision`
///   * `$HOME/.config/relision`
pub fn get_config_dir() -> String {
    if cfg!(target_os = "macos") {
        match env::home_dir() {
            Some(value) => {
                let mut result = value;
                result.push("Library");
                result.push("Application Support");
                result.push("relision");
                match result.into_os_string().into_string() {
                    Ok(rawpath) => {
                        return rawpath;
                    }
                    Err(_) => {}
                };
            }
            None => {}
        };
        panic!("Cannot locate configuration files for relision.  Please set HOME.");
    } else if cfg!(target_os = "windows") {
        match env::var_os("LOCALAPPDATA") {
            Some(value) => {
                match value.into_string() {
                    Ok(prefix) => {
                        return prefix + "\\relision";
                    }
                    Err(_) => {}
                };
            }
            None => {}
        };
        match env::var_os("USERPROFILE") {
            Some(value) => {
                match value.into_string() {
                    Ok(prefix) => {
                        return prefix + "\\AppData\\Local\\relision";
                    }
                    Err(_) => (),
                };
            }
            None => {}
        };
        match env::home_dir() {
            Some(value) => {
                let mut result = value;
                result.push("AppData");
                result.push("Local");
                result.push("relision");
                match result.into_os_string().into_string() {
                    Ok(res) => {
                        return res;
                    }
                    Err(_) => {}
                };
            }
            None => {}
        };
        panic!("Cannot locate configuration files for relision.  Please set HOME.");
    } else {
        match env::var_os("XDG_CONFIG_HOME") {
            Some(value) => {
                let mut result = value;
                result.push("/relision");
                match result.into_string() {
                    Ok(rawpath) => {
                        return rawpath;
                    }
                    Err(_) => {}
                };
            }
            None => {}
        };
        match env::home_dir() {
            Some(value) => {
                let mut result = value;
                result.push(".config");
                result.push("relision");
                match result.into_os_string().into_string() {
                    Ok(rawpath) => {
                        return rawpath;
                    }
                    Err(_) => {}
                };
            }
            None => {}
        };
        panic!("Cannot locate configuration files for relision.  Please set HOME.");
    }
}
