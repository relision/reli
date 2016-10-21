//! Write relision terms in the relision language.
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

use std::fmt;
use std::sync::Arc;
// This module depends on the term module and on the term writer module.
use super::terms::*;
use super::util::TermWriter;
use super::termfactory::TermFactory;
use std::ops::Deref;

/// Write a term in relision form.
pub struct EliWriter {}

impl TermWriter for EliWriter {
    fn new() -> Self {
        EliWriter {}
    }

    fn write(&self, form: &mut fmt::Formatter, fact: &TermFactory, term: &Arc<Term>) -> fmt::Result {
        match term.deref() {
            // The root term is unique among all terms.
            &Term::Root => write!(form, "^ROOT"),

            // Print a symbol literal.  If this is a known root type, just print it.
            // If the name is the same as a known root type, then print the type.
            // Otherwise only print the type if it is not SYMBOL.
            &Term::SymbolLiteral { ref locus, ref typ, ref value } => {
                // A symbol literal might denote a known term, or it might be a
                // simple symbol.
                if fact.is_named_root_term(value) {
                    if typ.deref() == fact.get_root().deref() {
                        // This is a named root term.
                        write!(form, "{}", value)
                    } else {
                        // Must print the type.
                        write!(form, "{}: ", value);
                        self.write(form, fact, typ)
                    }
                } else {
                    // This is not a named root term.  See if it's type is SYMBOL.
                    if typ.deref() == fact.get_symbol().deref() {
                        // No need to print the type.
                        write!(form, "{}", value)
                    } else {
                        // Must print the type.
                        write!(form, "{}: ", value);
                        self.write(form, fact, typ)
                    }
                }
            },

            &Term::StringLiteral { ref locus, ref typ, ref value } => {
                write!(form, "{:?}: {}", value, typ)
            },

            &Term::BooleanLiteral { ref locus, ref typ, ref value } => {
                write!(form, "{:?}: {}", value, typ)
            },

            &Term::Variable { ref locus, ref typ, ref name, ref guard } => {
                write!(form, "${}{{{}}}: {}", name, guard, typ)
            },

            &Term::StaticMap { ref locus, ref domain, ref codomain } => {
                write!(form, "{} => {}", domain, codomain)
            },

            &Term::StaticProduct { ref locus, ref lhs, ref rhs } => {
                write!(form, "{} * {}", lhs, rhs)
            },

            &Term::Lambda { ref locus, ref param, ref body, ref guard } => {
                write!(form, "{} ->{{{}}} {}", param, guard, body)
            },
        }
    }
}
