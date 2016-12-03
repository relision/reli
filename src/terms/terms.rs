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
use terms::locus::Locus;
use util::escape;

/// Define the different kinds of terms.
#[derive(Debug, PartialEq, PartialOrd)]
pub enum Term {
    /// The root term is the term that is its own type.  It is eternal,
    /// and its locus is "internal."
    Root,

    /// Define the symbol literal term.
    SymbolLiteral {
        /// The locus.
        locus: Locus,
        /// The type.
        typ: Arc<Term>,
        /// The value.
        value: String,
    },

    /// Define the string literal term.
    StringLiteral {
        /// The locus.
        locus: Locus,
        /// The type.
        typ: Arc<Term>,
        /// The value.
        value: String
    },

    /// Define the Boolean literal term.
    BooleanLiteral {
        /// The locus.
        locus: Locus,
        /// The type.
        typ: Arc<Term>,
        /// The value.
        value: bool,
    },

    /// Define the variable term.
    Variable {
        /// The locus.
        locus: Locus,
        /// The type.
        typ: Arc<Term>,
        /// The name.
        name: String,
        /// The guard.
        guard: Arc<Term>,
    },

    /// Define the static map.
    StaticMap {
        /// The locus.
        locus: Locus,
        /// The domain.
        domain: Arc<Term>,
        /// The co-domain.
        codomain: Arc<Term>,
    },

    /// Define the static product.
    StaticProduct {
        /// The locus.
        locus: Locus,
        /// The left-hand term.
        lhs: Arc<Term>,
        /// The right-hand term.
        rhs: Arc<Term>,
    },

    /// Define the lambda term.
    Lambda {
        /// The locus.
        locus: Locus,
        /// The lambda pattern.
        param: Arc<Term>,
        /// The lambda body.
        body: Arc<Term>,
        /// The lambda guard.
        guard: Arc<Term>,
    },

}

impl Term {
    // Methods that should be shared by all terms will go here.
}

fn with_locus(form: &mut fmt::Formatter, loc: &Locus) -> fmt::Result {
    if *loc != Locus::Internal {
        write!(form, " /* {} */", loc)
    } else {
        Ok(())
    }
}

// Provide a default visualization for terms.  This is not quite the same as the ELI
// format that will be defined elsewhere.
use std::fmt;
impl fmt::Display for Term {
    fn fmt(&self, form: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Term::Root => write!(form, "^ROOT"),
            Term::SymbolLiteral { ref locus, ref typ, ref value } => {
                let (escaped,fixed) = escape(value, '`');
                if fixed {
                    try!(write!(form, "`{}`", escaped));
                } else {
                    try!(write!(form, "{}", escaped));
                }
                if **typ != Term::Root {
                    try!(write!(form, ": {}", typ));
                }
                with_locus(form, locus)
            },
            Term::StringLiteral { ref locus, ref typ, ref value } => {
                let (escaped,_) = escape(value, '"');
                try!(write!(form, "\"{}\": {}", escaped, typ));
                with_locus(form, locus)
            },
            Term::BooleanLiteral { ref locus, ref typ, ref value } => {
                try!(write!(form, "{:?}: {}", value, typ));
                with_locus(form, locus)
            },
            Term::Variable { ref locus, ref typ, ref name, ref guard } => {
                let (escaped,fixed) = escape(name, '`');
                if fixed {
                    try!(write!(form, "$`{}`", escaped));
                } else {
                    try!(write!(form, "${}", escaped));
                }
                try!(write!(form, "{{{}}}: {}", guard, typ));
                with_locus(form, locus)
            },
            Term::StaticMap { ref locus, ref domain, ref codomain } => {
                try!(write!(form, "{} => {}", domain, codomain));
                with_locus(form, locus)
            },
            Term::StaticProduct { ref locus, ref lhs, ref rhs } => {
                try!(write!(form, "{} * {}", lhs, rhs));
                with_locus(form, locus)
            },
            Term::Lambda { ref locus, ref param, ref body, ref guard } => {
                try!(write!(form, "{} ->{{{}}} {}", param, guard, body));
                with_locus(form, locus)
            },
        }
    }
}
