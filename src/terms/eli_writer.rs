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

use std::io;
use std::sync::Arc;
// This module depends on the term module and on the term writer module.
use super::terms::*;
use super::util::TermWriter;
use super::termfactory::TermFactory;
use std::ops::Deref;
use util::escape;

/// Write a term in relision form.
pub struct EliWriter {}

impl TermWriter for EliWriter {
    fn new() -> Self {
        EliWriter {}
    }

    fn print(&self, fact: &TermFactory, term: &Arc<Term>) {
        self.write(&mut io::stdout(), fact, term).unwrap();
    }

    fn println(&self, fact: &TermFactory, term: &Arc<Term>) {
        self.write(&mut io::stdout(), fact, term).unwrap();
        println!("");
    }

    fn write(&self, dest: &mut io::Write, fact: &TermFactory, term: &Arc<Term>) -> io::Result<()> {
        match term.deref() {
            // The root term is unique among all terms.
            &Term::Root => write!(dest, "^ROOT"),

            // Print a symbol literal.  If this is a known root type, just print it.
            // If the name is the same as a known root type, then print the type.
            // Otherwise only print the type if it is not SYMBOL.
            &Term::SymbolLiteral { ref typ, ref value, .. } => {
                // A symbol literal might denote a known term, or it might be a
                // simple symbol.
                let (mut escaped,modified) = escape(value, '`');
                if modified {
                    escaped = format!("`{}`", escaped);
                }
                if fact.is_named_root_term(value) {
                    try!(write!(dest, "{}", escaped));
                    if typ.deref() != fact.get_root().deref() {
                        // Must print the type.
                        try!(write!(dest, ": "));
                        self.write(dest, fact, typ)
                    } else {
                        Ok(())
                    }
                } else {
                    // This is not a named root term.  See if it's type is SYMBOL.
                    try!(write!(dest, "{}", escaped));
                    if typ.deref() != fact.get_symbol().deref() {
                        // Must print the type.
                        try!(write!(dest, ": "));
                        self.write(dest, fact, typ)
                    } else {
                        Ok(())
                    }
                }
            },

            &Term::StringLiteral { ref typ, ref value, .. } => {
                let (escaped,_) = escape(value, '"');
                write!(dest, "\"{}\"", escaped).unwrap();
                if typ.deref() != fact.get_string().deref() {
                    try!(write!(dest, ": "));
                    self.write(dest, fact, typ)
                } else {
                    Ok(())
                }
            },

            &Term::BooleanLiteral { ref typ, ref value, .. } => {
                write!(dest, "{:?}", value).unwrap();
                if typ.deref() != fact.get_boolean().deref() {
                    try!(write!(dest, ": "));
                    self.write(dest, fact, typ)
                } else {
                    Ok(())
                }
            },

            &Term::Variable { ref typ, ref name, ref guard, .. } => {
                let (escaped,modified) = escape(name, '`');
                if modified {
                    try!(write!(dest, "$`{}`", escaped));
                } else {
                    try!(write!(dest, "${}", escaped));
                }
                match guard.deref() {
                    &Term::BooleanLiteral { ref typ, value, .. } => {
                        if typ.deref() != fact.get_boolean().deref() || value != true {
                            try!(write!(dest, "{{"));
                            try!(self.write(dest, fact, guard));
                            try!(write!(dest, "}}"));
                        }
                    }
                    _ => {
                        try!(write!(dest, "{{"));
                        try!(self.write(dest, fact, guard));
                        try!(write!(dest, "}}"));
                    }
                }
                if typ.deref() != fact.get_any().deref() {
                    try!(write!(dest, ": "));
                    self.write(dest, fact, typ)
                } else {
                    Ok(())
                }
            },

            &Term::StaticMap { ref domain, ref codomain, .. } => {
                try!(self.write(dest, fact, domain));
                try!(write!(dest, " => "));
                self.write(dest, fact, codomain)
            },

            &Term::StaticProduct { ref lhs, ref rhs, .. } => {
                try!(self.write(dest, fact, lhs));
                try!(write!(dest, " * "));
                self.write(dest, fact, rhs)
            },

            &Term::Lambda { ref param, ref body, ref guard, .. } => {
                try!(self.write(dest, fact, param));
                try!(write!(dest, " ->"));
                match guard.deref() {
                    &Term::BooleanLiteral { ref typ, value, .. } => {
                        if typ.deref() != fact.get_boolean().deref() || value != true {
                            try!(write!(dest, "{{"));
                            try!(self.write(dest, fact, guard));
                            try!(write!(dest, "}}"));
                        }
                    }
                    _ => {
                        try!(write!(dest, "{{"));
                        try!(self.write(dest, fact, guard));
                        try!(write!(dest, "}}"));
                    }
                }
                try!(write!(dest, " "));
                self.write(dest, fact, body)
            },
        }
    }
}
