// Copyright (c) 2015 by Stacy Prowell.  All rights reserved.
//
// Licensed under the BSD 2-Clause license.  See the file LICENSE
// that is part of this distribution.  This file may not be copied,
// modified, or distributed except according to those terms.extern

//! Provide basic definitions of terms.

use num::BigInt;
use std::rc::Rc;

mod root;

/// Export the kinds of terms.
pub use self::TermKind::{RootKind};

/// Represent the kind of a term.
pub enum TermKind {
	/// The special root type.
	RootKind,
}

trait Term {
	fn get_type(&self) -> Rc<Term>;
	fn get_kind(&self) -> TermKind;
}