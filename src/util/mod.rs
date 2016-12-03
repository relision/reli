//! Provide some general helper functions that do not depend on any other module.
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

/// Given a string and a "border" character, properly escape special characters in the string.
/// Special characters are exactly the following.
///
/// * nul (U+0000) becomes \0
/// * tabulator (U+0009) becomes \t
/// * newline (U+000A) becomes \a
/// * carriage return (U+000D) becomes \r
/// * backslash or reverse solidus (U+005C) becomes \\
///
/// Additionally characters in the range 0x80 - 0xFF are rendered as \xHH, where HH are the two
/// hexadecimal characters of the character value.  Characters above 0x0000FF are rendered using
/// the Rust standard of \u{HHHHHH}, where HHHHHH are up to six hexadecimal digits.
///
/// Finally, if the character specified by `border` is found, it is also escaped.
///
/// The escaped string is returned.  Note that hex digits are capitalized.
pub fn escape(input: &String, border: char) -> (String, bool) {
    let mut output = String::new();
    let mut fixed = false;
    for ch in input.chars() {
        if ch == border {
            output.push_str("\\");
            output.push(border);
            fixed = true;
            continue;
        }
        if (ch as u32) > 127 {
            if (ch as u32) < 256 {
                output.push_str(format!("\\x{:2X}", ch as u8).as_str());
            } else {
                output.push_str(format!("\\u{{{:4X}}}", ch as u32).as_str());
            }
            fixed = true;
            continue;
        }
        match ch {
            '\0' => { output.push_str("\\0"); fixed = true },
            '\t' => { output.push_str("\\t"); fixed = true },
            '\n' => { output.push_str("\\n"); fixed = true },
            '\r' => { output.push_str("\\r"); fixed = true },
            '\\' => { output.push_str("\\\\"); fixed = true },
            _ => output.push(ch)
        };
    }
    (output, fixed)
}
