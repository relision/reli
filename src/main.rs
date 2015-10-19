//!           _ _     _
//!  _ __ ___| (_)___(_) ___  _ __
//! | '__/ _ \ | / __| |/ _ \| '_ \
//! | | |  __/ | \__ \ | (_) | | | |
//! |_|  \___|_|_|___/_|\___/|_| |_|
//! The relision term rewriting library.
//!
//! # License
//!
//! Copyright (c) 2015 by Stacy Prowell.  All rights reserved.
//!
//! Licensed under the BSD 2-Clause license.  See the file LICENSE
//! that is part of this distribution.  This file may not be copied,
//! modified, or distributed except according to those terms.

extern crate linenoise;
extern crate getopts;
extern crate num;
extern crate relision;
use getopts::Options;
use std::env;

/// The REPL.
fn repl() {
  let mut history_filename = relision::get_config_dir();
  history_filename.push_str("/repl.history");
  // We got our filename. Drop the mutability.
  let history_filename = history_filename;
  linenoise::history_load(&history_filename);
  loop {
    let val = linenoise::input("e> ");
    match val {
        None => {
            linenoise::history_save(&history_filename);
            break;
        }
        Some(input) => {
            println!("{}", input);
            linenoise::history_add(&input);
            if input == "clear" {
              linenoise::clear_screen();
            }
        }
    } // Match.
  } // REPL loop.
  println!("Terminating REPL.");
}

/// Print the command line help.  First print the prototype for using the
/// command, and then print help about using the switches.
/// progname: The program name.
/// switches: The allowed command line switch data structure.
fn print_usage(progname: &str, switches: Options) {
    let prototype = format!("Usage: {} [switches...] [elision files...]", progname);
    print!("{}", switches.usage(&prototype));
}

/// Print the banner for the project.
fn banner() {
    println!(r#"
          _ _     _
 _ __ ___| (_)___(_) ___  _ __
| '__/ _ \ | / __| |/ _ \| '_ \
| | |  __/ | \__ \ | (_) | | | |
|_|  \___|_|_|___/_|\___/|_| |_|
The relision term rewriting library.

"#)
}

/// Entry point when run from the prompt.
fn main() {
    banner();

    println!("Running on {}.", relision::get_platform());
    println!("Configuration stored at: {}.", relision::get_config_dir());

    // Get the command line arguments.
    let args: Vec<String> = env::args().collect();
    let me = args[0].clone();

    // Specify the switches this wrapper takes.
    let mut switches = Options::new();
    switches.optflag("h", "help", "Print this command line help.");

    // Now process all command line switches.  The "tail" removes the program
    // name.
    let matches = match switches.parse(&args[1..]) {
        Ok(mat) => { mat }
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

    // Now run the REPL.
    repl();
}
