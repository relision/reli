//! Implement the REPL.
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

extern crate linenoise;
extern crate num;
extern crate getopts;

/// The return result from a callback.  Errors are returned as a string in an `Err` result, while
/// success is indicated by `Ok("".to_owned())`.
pub type CallbackResult = Result<String, String>;

/// A callback is a function mapping a vector of strings to a return value.  The argument to the
/// function is the vector of text arguments given at invocation.  The result indicates success or
/// an error.
pub type Callback = Box<Fn(Vec<&str>) -> CallbackResult>;

/// Struct to hold a colon command definition.
pub struct ColonCommand {
    /// The name of the command, matched by first unique prefix.
    name: &'static str,
    /// The action to perform when the command is detected.
    action: Callback,
}

/// Obtain the supported colon commands, each mapped to a closure that
/// executes the command.
pub fn define_commands() -> Vec<ColonCommand> {
    // TODO This should really be a map.  Need to change that at some point.
    vec![ColonCommand {
             name: "clear",
             action: Box::new(|_| -> CallbackResult {
                 linenoise::clear_screen();
                 Ok("".to_string())
             }),
         },
         ColonCommand {
             name: "history",
             action: Box::new(|_| -> CallbackResult {
                 let mut index = 0;
                 loop {
                     match linenoise::history_line(index) {
                         None => {
                             break;
                         }
                         Some(line) => {
                             println!("{}: {}", index, line);
                         }
                     };
                     index = index + 1;
                 }
                 Ok("".to_string())
             }),
         },
         ColonCommand {
             // This command is special.  It is trapped by the main loop and
             // causes the REPL to terminate.
             name: "quit",
             action: Box::new(|_| -> CallbackResult { Ok("".to_string()) }),
         }]
}

/// Create a REPL backed by a history file.  Prior history is read at start, and then saved at the
/// end.  The argument must be the full path to the file storing the history.
///
///   * `history_filename`: The path to the file where history is stored.
///
/// Note: The history file must be both readable and writeable.
pub fn backed_repl(history_filename: &str) {
    linenoise::history_load(&history_filename);
    repl(define_commands());
    linenoise::history_save(&history_filename);
}

/// Implementation of an interactive REPL.
///
///   * `commands`: An array defining all colon commands.
///
pub fn repl(commands: Vec<ColonCommand>) {
    'repl: loop {
        let val = linenoise::input("e> ");
        match val {
            None => {
                break;
            }
            Some(input) => {
                // Accumulate history but ignore empty lines.
                let input = input.trim();
                if input == "" {
                    continue 'repl;
                }
                linenoise::history_add(&input);

                // Check for, and process, any colon commands.  We allow commands
                // to be abbreviated to just the first few characters, but this
                // must be unambiguous.
                if input.starts_with(":") {
                    // Found a potential colon command, so now process it.
                    let arguments: Vec<&str> = input.split(' ').collect();
                    let mut possible: Option<&ColonCommand> = None;
                    let mut found = false;
                    let command = arguments[0].clone();
                    let command = &command[1..];
                    for cmd in &commands {
                        if cmd.name.starts_with(command) {
                            // Found a possible answer.
                            match possible {
                                None => {
                                    possible = Some(cmd);
                                    found = true;
                                }
                                Some(other) => {
                                    println!("ERROR: The command :{} is ambiguous.  It might be \
                                              :{} or :{}.",
                                             command,
                                             cmd.name,
                                             other.name);
                                    break;
                                }
                            }
                        }
                    } // Iterate over commands.
                    if !found {
                        println!("ERROR: The command :{} was not recognized.", command);
                    } else {
                        // We found the command (possibly) and we need to execute it, passing along
                        // all the other arguments.  The return will be either Ok (if all went well),
                        // or Err (if something went wrong), and we need to print the error message.
                        match possible {
                            None => {}
                            Some(other) => {
                                // Terminate on the magic quit command.
                                if other.name == "quit" {
                                    let answer = linenoise::input("Really quit? (y/n) ");
                                    match answer {
                                        Some(input) => {
                                            if input.starts_with("y") || input.starts_with("Y") {
                                                break 'repl;
                                            }
                                        }
                                        None => {}
                                    }
                                }
                                match (&other.action)(arguments) {
                                    Ok(_) => {}
                                    Err(msg) => {
                                        println!("ERROR: {}", msg);
                                    }
                                }
                            }
                        }
                    }
                } else {
                    // Not a colon command.
                    println!("  -> {}", input);
                }
            }
        } // Match.
    } // REPL loop.
    println!("Terminating REPL.");
}

/// Print the banner for the project.
pub fn banner() {
    println!(r#"
          _ _     _
 _ __ ___| (_)___(_) ___  _ __
| '__/ _ \ | / __| |/ _ \| '_ \
| | |  __/ | \__ \ | (_) | | | |
|_|  \___|_|_|___/_|\___/|_| |_|
The relision term rewriting library.
"#);
    println!("Version: {:?}",
             option_env!("CARGO_PKG_VERSION").unwrap_or("unspecified"));
}
