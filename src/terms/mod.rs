// Copyright (c) 2015 by Stacy Prowell.  All rights reserved.
//
// Licensed under the BSD 2-Clause license.  See the file LICENSE
// that is part of this distribution.  This file may not be copied,
// modified, or distributed except according to those terms.extern

//! Provide basic definitions of terms.

/// Represent an instance of a term.
#[derive(Copy, Eq, Debug)]
pub enum Term {
	Root,
	Any,
	None,
	Symbol(Box<Term>, String),
	String(Box<Term>, String),
	Integer(Box<Term>, i64),
	Float(Box<Term>, f64),
	BitString(Box<Term>, i64, i64),
	Boolean(Box<Term>, bool),
}
