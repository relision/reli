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
use relision::os_spec;

/* A block of code to figure out the current build platform.  The idea is that
 * we figure out the platform and then make it available at runtime.  Is there
 * a Posix way to do this?
 *
 * Why do we care?  This determines - among other things - where to find the
 * configuration files.
 */

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
  println!("Running on {}.", os_spec::tos());
  println!("Configuration stored at: {}.", os_spec::get_config_dir());

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
