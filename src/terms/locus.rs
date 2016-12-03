//! Provide a location.
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

/// The location of a term's declaration.
/// Every term can have an associated location, which tells where the term
/// was originally declared.  This can be *internal*, or it can be from a
/// *file*, or it could be from an interactive *console* session.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Locus {
	/// The internal locus applies where a term is created not as the result of
	/// a file or console input line.
	Internal,
	/// The console locus applies where a term is created as the result of an
	/// input line without a known relevant file.  This could be interactive,
	/// or it could be from an unnamed stream.
	Console(u32, u32),
	/// The file locus applies where a file (or other named source like a URL)
	/// is the source of the term.
	File(String, u32, u32),
}

use std::fmt;
impl fmt::Display for Locus {
	fn fmt(&self, form: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Locus::Internal => write!(form, ""),
			Locus::Console(line, column) => {
				write!(form, "{}:{}", line, column)
			},
			Locus::File(ref name, line, column) => {
				write!(form, "{}:{}:{}", name, line, column)
			}
		}
	}
}
