//! Define terms.
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

use std::sync::Arc;

/// Define the different kinds of terms.
#[derive(Debug, PartialEq, PartialOrd)]
pub enum Term {
    /// The root term is the term that is its own type.
    Root,

    /// Define the symbol literal term.
    SymbolLiteral {
        /// The type.
        typ: Arc<Term>,
        /// The value.
        value: String,
    },

    /// Define the string literal term.
    StringLiteral {
        /// The type.
        typ: Arc<Term>,
        /// The value.
        value: String
    },

    /// Define the Boolean literal term.
    BooleanLiteral {
        /// The type.
        typ: Arc<Term>,
        /// The value.
        value: bool,
    },

    /// Define the variable term.
    Variable {
        /// The type.
        typ: Arc<Term>,
        /// The name.
        name: String,
        /// The guard.
        guard: Arc<Term>,
    },

    /// Define the static map.
    StaticMap {
        /// The domain.
        domain: Arc<Term>,
        /// The co-domain.
        codomain: Arc<Term>,
    },

    /// Define the static product.
    StaticProduct {
        /// The left-hand term.
        lhs: Arc<Term>,
        /// The right-hand term.
        rhs: Arc<Term>,
    },

    /// Define the lambda term.
    Lambda {
        /// The lambda pattern.
        param: Arc<Term>,
        /// The lambda body.
        body: Arc<Term>,
        /// The lambda guard.
        guard: Arc<Term>,
    },

}

impl Term {
}

use std::fmt;
impl fmt::Display for Term {
    fn fmt(&self, form: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Term::Root => write!(form, "^ROOT"),
            Term::SymbolLiteral { ref typ, ref value } => {
                write!(form, "{}: {}", value, typ)
            },
            Term::StringLiteral { ref typ, ref value } => {
                write!(form, "{:?}: {}", value, typ)
            },
            Term::BooleanLiteral { ref typ, ref value } => {
                write!(form, "{:?}: {}", value, typ)
            },
            Term::Variable { ref typ, ref name, ref guard } => {
                write!(form, "${}{{{}}}: {}", name, guard, typ)
            },
            Term::StaticMap { ref domain, ref codomain } => {
                write!(form, "{} => {}", domain, codomain)
            },
            Term::StaticProduct { ref lhs, ref rhs } => {
                write!(form, "{} * {}", lhs, rhs)
            },
            Term::Lambda { ref param, ref body, ref guard } => {
                write!(form, "{} ->{{{}}} {}", param, guard, body)
            },
        }
    }
}
