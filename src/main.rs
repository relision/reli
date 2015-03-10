extern crate linenoise;

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

fn main() {
  println!("Running on {}.", tos());
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
    }
  }
}
