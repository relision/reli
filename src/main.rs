// Copyright (c) 2015 by Stacy Prowell.  All rights reserved.
//
// Licensed under the BSD 2-Clause license.  See the file LICENSE
// that is part of this distribution.  This file may not be copied,
// modified, or distributed except according to those terms.extern

#![feature(collections,core)]

extern crate linenoise;
extern crate getopts;
extern crate num;
extern crate relision;
use getopts::Options;

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
fn main() {
  println!("Running on {}.", relision::get_platform());
  println!("Configuration stored at: {}.", relision::get_config_dir());

  // Get the command line arguments.
  let args = std::env::args().collect::<Vec<String>>();
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
