//! Utility definitions for terms.
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

use std::io;
use std::sync::Arc;
use super::terms::*;
use super::termfactory::TermFactory;

/// Write a term.
pub trait TermWriter {
    /// Make a new instance.
    fn new() -> Self;

    /// Print directly to standard out.
    fn print(&self, fact: &TermFactory, term: &Arc<Term>);

    /// Print directly to standard out.
    fn println(&self, fact: &TermFactory, term: &Arc<Term>);

    /// Write a term to the given formatter.
    fn write(&self, dest: &mut io::Write, fact: &TermFactory, term: &Arc<Term>) -> io::Result<()>;
}
