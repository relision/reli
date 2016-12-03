//! Provide facilities for defining and working with terms.
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

/*
This module provides the facilities for constructing and working with terms.

Becuse terms are shared across data structures, and because the library is multi-threaded, every
term must be wrapped in an Arc.  Thus Arc<Term> is the way terms are returned.  The model is
"clone on pass," which means that when a term is passed to a routine, it is cloned and the
"clone" is passed (really the ref count is just bumped).
*/

mod terms;          // The different kinds of terms.
mod termfactory;    // Constructing terms.
mod util;           // Utilities for working with terms.
mod eli_writer;     // Write terms in ELI format.
mod universe;       // The term universe.
mod locus;          // The locus.

// Expose the important stuff directly through this module to simplify the
// interface.
pub use self::locus::Locus;
pub use self::terms::Term;
pub use self::termfactory::TermFactory;
pub use self::util::TermWriter;
pub use self::eli_writer::EliWriter;
