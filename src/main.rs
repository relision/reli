//! Command line access to the REPL.
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

extern crate relision;
extern crate getopts;
use getopts::Options;
use std::env;

// Get the module implementing the REPL functions.
mod repl;

/// Print the command line help.  First print the prototype for using the
/// command, and then print help about using the switches.
///   * `progname`: The program name.
///   * `switches`: The allowed command line switch data structure.
fn print_usage(progname: &str, switches: Options) {
    let prototype = format!("Usage: {} [switches...] [elision files...]", progname);
    print!("{}", switches.usage(&prototype));
}

/// Entry point when run from the prompt.
fn main() {
    // Get the command line arguments.
    let args: Vec<String> = env::args().collect();
    let me = args[0].clone();

    // Specify the switches this wrapper takes.
    let mut switches = Options::new();
    switches.optflag("h", "help", "Print this command line help.");

    // Now process all command line switches.  The "tail" removes the program
    // name.
    let matches = match switches.parse(&args[1..]) {
        Ok(mat) => mat,
        Err(fail) => {
            println!("ERROR parsing command line arguments:");
            println!("  {}", fail.to_string());
            return;
        }
    };
    if matches.opt_present("h") {
        print_usage(&me, switches);
        return;
    }

    // Print the banner.
    repl::banner();
    println!("Running on {}.", relision::platform::get_platform());
    let config_dir = relision::platform::get_config_dir();
    println!("Configuration stored at: {}.", config_dir);

    // Now run the REPL.
    let history_filename = config_dir + ("/repl.history");
    repl::backed_repl(&history_filename);
}
