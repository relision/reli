// Copyright (c) 2015 by Stacy Prowell.  All rights reserved.
//
// Licensed under the BSD 2-Clause license.  See the file LICENSE
// that is part of this distribution.  This file may not be copied,
// modified, or distributed except according to those terms.extern

//! Provide a location.

extern crate lazy_static;

/// The location of a term's declaration.
/// Every term can have an associated location, which tells where the term
/// was originally declared.  This can be *internal*, or it can be from a
/// *file*, or it could be from an interactive *console* session.
///
/// To make a location, do one of the following.
///   - If the location is from `file` with line number `line` and column
///     number `column`, then invoke `Loc(file, line, column)`.
///   - If the location is from a console session with line number `line` and
///     column number `column`, then invoke `Loc(line, column)`.
///   - If the term is being created as part of an internal process (such as
///     the root type) or otherwise has no associated location, then get the
///     internal location via `Loc::get_internal()`.
// #[derive(Debug)]
// pub enum Locus2 {
// 	Internal,
// 	Console(line: u32, column: u32),
// 	File(name: String, line: u32, column: u32),
// }


#[derive(Debug)]
pub struct Locus {
	source: String,
	line: u32,
	column: u32,
}

lazy_static! {
	static ref INTERNAL: Locus = Locus {
		source: "internal".to_string(),
		line: 0,
		column: 0,
	};
	static ref CONSOLE: String = "console".to_string();
}

impl Locus {
	/// Make a new locus.
	fn new(src: String, ln: u32, cn: u32) -> Locus {
		Locus{
			source: src, line: ln, column: cn,
		}
	}
	/// Make a new console locus.
	fn console(ln: u32, cn: u32) -> Locus {
		Locus {
			source: *CONSOLE, line: ln, column: cn,
		}
	}
	/// Get the internal locus.
	fn get_internal() -> Locus {
		*INTERNAL
	}
	/// Get the source.
	fn get_source(&self) -> String {
		self.source.clone()
	}
	/// Get the line.
	fn get_line(&self) -> u32 {
		self.line
	}
	/// Get the column.
	fn get_column(&self) -> u32 {
		self.column
	}
}
