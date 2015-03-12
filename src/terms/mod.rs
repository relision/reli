// Copyright (c) 2015 by Stacy Prowell.  All rights reserved.
//
// Licensed under the BSD 2-Clause license.  See the file LICENSE
// that is part of this distribution.  This file may not be copied,
// modified, or distributed except according to those terms.extern

//! Provide basic definitions of terms.

use num::BigInt;
use std::rc::Rc;

/// Export the kinds of terms.
pub use self::Term::{Root, SymbolLiteral, StringLiteral, IntegerLiteral,
	FloatLiteral, BitStringLiteral, BooleanLiteral};

/// Represent an instance of a term.
pub enum Term {
	/// The special root type.
	Root,
	/// Integer literals.
	IntegerLiteral(Rc<Term>, BigInt),
	/// Float literals.
	FloatLiteral(Rc<Term>, BigInt, BigInt, u32),
	/// Bit string literals.
	BitStringLiteral(Rc<Term>, BigInt, BigInt),
	/// Boolean literals.
	BooleanLiteral(Rc<Term>, bool),
	/// Symbol literals.
	SymbolLiteral(Rc<Term>, String),
	/// String literals.
	StringLiteral(Rc<Term>, String),
}
