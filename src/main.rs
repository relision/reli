// Copyright (c) 2015 by Stacy Prowell.  All rights reserved.
//
// Licensed under the BSD 2-Clause license.  See the file LICENSE
// that is part of this distribution.  This file may not be copied,
// modified, or distributed except according to those terms.extern

extern crate linenoise;
extern crate getopts;
extern crate num;
extern crate relision;
use getopts::Options;

/* A block of code to figure out the current build platform.  The idea is that
 * we figure out the platform and then make it available at runtime.  Is there
 * a Posix way to do this?
 *
 * Why do we care?  This determines - among other things - where to find the
 * configuration files.
 */

#[cfg(target_os = "linux")]
/// Report the target compilation operating system.
fn tos() -> &'static str {
  "linux"
}
#[cfg(target_os = "win32")]
/// Report the target compilation operating system.
fn tos() -> &'static str {
  "win32"
}
#[cfg(target_os = "macos")]
/// Report the target compilation operating system.
fn tos() -> &'static str {
  "macos"
}
#[cfg(not(any(target_os = "macos",
              target_os = "win32",
              target_os = "linux")))]
/// Report the target compilation operating system.
fn tos() -> &'static str {
  "unknown"
}

/// The REPL.
fn repl() {
  loop {
    let val = linenoise::input("e> ");
    match val {
        None => { break }
        Some(input) => {
            println!("{}", input);
            linenoise::history_add(input.as_slice());
            if input.as_slice() == "clear" {
              linenoise::clear_screen();
            }
        }
    } // Match.
  } // REPL loop.
}

/// Print the command line help.  First print the prototype for using the
/// command, and then print help about using the switches.
/// progname: The program name.
/// switches: The allowed command line switch data structure.
fn print_usage(progname: &str, switches: Options) {
  let prototype = format!("Usage: {} [switches...] [elision files...]", progname);
  print!("{}", switches.usage(&prototype));
}

/// Entry point when run from the prompt.
///
/// # Panics
/// The command line arguments do not parse correctly.
fn main() {
  println!("Running on {}.", tos());

  // Get the command line arguments.
  let args: Vec<String> = std::os::args();
  let me = args[0].clone();

  // Specify the switches this wrapper takes.
  let mut switches = getopts::Options::new();
  switches.optflag("h", "help", "Print this command line help.");

  // Now process all command line switches.  The "tail" removes the program
  // name.
  let matches = match switches.parse(args.tail()) {
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
