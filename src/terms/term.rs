// Copyright (c) 2015 by Stacy Prowell.  All rights reserved.
//
// Licensed under the BSD 2-Clause license.  See the file LICENSE
// that is part of this distribution.  This file may not be copied,
// modified, or distributed except according to those terms.extern

/// Define a type for terms.
type Term = std::rc::Rc<iTerm>;

/// Provide the elements common to all terms.
trait iTerm {

	/// Get the type of this term.
	fn get_type() -> Rc<Term>;

	/// Get a string representation of this term.
	fn to_string() -> String;

	/// Get the hash code for this term.
	fn get_hash() -> u32;

	/// Get the de Bruijn index for this term.
	///
	/// De Bruijn indices are used for lambdas to prevent "capturing" free
	/// variables prematurely.  See
	/// https://en.wikipedia.org/wiki/De_Bruijn_index.
	///
	/// Terms that do not contain variables bound in lambdas will have an
	/// index of zero.  Otherwise the index will be positive.
	fn get_de_bruijn_index() -> u32;

	/// Get the depth of this term.  The depth of the root is zero; otherwise
	/// the depth is one plus the maximum depth of all child terms, with a few
	/// exceptions noted below.
	///
	/// Depth is used in matching, with the intent that if the pattern depth
	/// exceeds the subject depth, no match is possible.  For this reason the
	/// depth of the type is included, but the depth of the guards (for
	/// variables and lambdas) is not.
	fn get_depth() -> u32;

	/// Determine if this is a metaterm or not.  A term is a *metaterm* if it
	/// contains a term variable or a metaterm.
	fn is_meta_term() -> bool;

	/// Determine if this term is a constant.  A constant term consists of just
	/// a literal or combinations of literals and constants, and has no
	/// variables.  This includes any form of lambda who's body is a constant,
	/// even though the lambda contains a variable for its parameter.
	fn is_constant() -> bool;

	
}
