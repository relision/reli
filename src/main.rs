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

extern crate relision;      // Get the relision crate (the library).
extern crate getopts;       // Use an external crate to process command line options.

use getopts::Options;
use std::env;

// Get the module implementing the REPL functions.
use relision::repl;

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
    // Go and get the locus stuff.
    use relision::terms::locus::Locus;
    let locus1 = Locus::Internal;
    let locus2 = Locus::Console(9, 21);
    let locus3 = Locus::File("brenda.eli".to_string(), 9, 21);
    let locus4 = Locus::File("brenda.eli".to_string(), 9, 21);
    println!("locus1: {}\nlocus2: {}\nlocus3: {}\nlocus4: {}\n", locus1, locus2, locus3, locus4);
    println!("{} == {} -> {}", locus3, locus4, locus3 == locus4);
    println!("{} == {} -> {}", locus2, locus3, locus2 == locus3);

    // use std::sync::Arc;
    // use relision::terms::Term;
    use relision::terms::TermFactory;
    use relision::terms::{TermWriter, EliWriter};
    let fact = TermFactory::new();
    let _eli = EliWriter::new();
    let t = fact.get_root();
    println!("{}\n    {:?}", t, t);
    let u = fact.new_string("Mister Pickles".to_string());
    println!("{}\n    {:?}", u, u);
    let v = fact.new_variable(&fact.get_symbol(), "vivian".to_string(), &fact.new_boolean(true));
    println!("{}\n    {:?}", v, v);
    let m = fact.new_static_map(&v, &u);
    println!("{}\n    {:?}", m, m);
    let p = fact.new_static_product(&fact.get_string(), &fact.get_symbol());
    println!("{}\n    {:?}", p, p);
    let l = fact.new_lambda(&v, &m, &fact.new_boolean(true));
    println!("{}\n    {:?}", l, l);
    println!("{}\n    {:?}", fact.get_type(&l), fact.get_type(&l));

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
