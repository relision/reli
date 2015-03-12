// Copyright (c) 2015 by Stacy Prowell.  All rights reserved.
//
// Licensed under the BSD 2-Clause license.  See the file LICENSE
// that is part of this distribution.  This file may not be copied,
// modified, or distributed except according to those terms.extern

//! Provide basic definitions of terms.

use num::BigInt;
use std::rc::Rc;

/// Export the kinds of terms.
pub use self::Term::{Root, /*Symbol, String,*/ Integer, Float, BitString, Boolean};

/// Represent an instance of a term.
#[derive(Copy, Debug)]
pub enum Term {
	/// The special root type.
	Root,
	/// Integer literals.
	Integer(Rc<Term>, BigInt),
	/// Float literals.
	Float(Rc<Term>, BigInt, BigInt, u32),
	/// Bit string literals.
	BitString(Rc<Term>, BigInt, BigInt),
	/// Boolean literals.
	Boolean(Rc<Term>, bool),
	// /// Symbol literals.
	// Symbol(Rc<Term>, String),
	// /// String literals.
	// String(Rc<Term>, String),
}

impl Term {
	pub fn get_typ(&self) -> Term {
		match *self {
			Root => Root,
			// Symbol(typ, _) => typ,
			// String(typ, _) => typ,
			Integer(typ, _) => typ,
			Float(typ, _, _, _) => typ,
			BitString(typ, _, _) => typ,
			Boolean(typ, _) => typ
		}
	}
}
