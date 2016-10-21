//! Define a factory to build terms and obtain well-known terms.
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

// This module depends on the terms module.
use super::terms::*;
use std::sync::Arc;
use std::collections::HashMap;
use terms::locus::Locus;

/// The term factory.
pub struct TermFactory {

    /*
     * The known root terms.
     */

    /// Root.
    the_root: Arc<Term>,
    /// SYMBOL type.
    the_symbol: Arc<Term>,
    /// STRING type.
    the_string: Arc<Term>,
    /// INTEGER type.
    the_integer: Arc<Term>,
    /// FLOAT type.
    the_float: Arc<Term>,
    /// BIT_STRING type.
    the_bit_string: Arc<Term>,
    /// BOOLEAN type.
    the_boolean: Arc<Term>,
    /// ANY type.
    the_any: Arc<Term>,
    /// NONE type.
    the_none: Arc<Term>,
    /// MAP type.
    the_map: Arc<Term>,
    /// PRODUCT type.
    the_product: Arc<Term>,
    /// SPECIAL_FORM type.
    the_special_form: Arc<Term>,
    /// PROPERTIES type.
    the_properties: Arc<Term>,

    /*
     * Well-known constants.
     */

    /// The Boolean true.
    is_true: Arc<Term>,
    /// The Boolean false.
    is_false: Arc<Term>,

    /*
     * Stored information.
     */

    /// Well-known named root terms.
    named_terms: HashMap<String, Arc<Term>>,
}

/// Local macro to create a new root type.
macro_rules! nrt {
    ($typ: expr, $name: expr) => {
        Arc::new(Term::SymbolLiteral {
            locus: Locus::Internal,
            typ: $typ.clone(),
            value: $name
        })
    }
}

impl TermFactory {
    /// Construct and return a new term factory.
    pub fn new() -> Self {
        let root = Arc::new(Term::Root);
        let the_boolean = Arc::new(Term::SymbolLiteral {
            locus: Locus::Internal, typ: root.clone(), value: "BOOLEAN".to_string()
        });
        let mut fact = TermFactory {
            // Initialize the root terms.
            the_root: root.clone(),
            the_symbol: nrt!(root, "SYMBOL".to_string()),
            the_string: nrt!(root, "STRING".to_string()),
            the_integer: nrt!(root, "INTEGER".to_string()),
            the_float: nrt!(root, "FLOAT".to_string()),
            the_bit_string: nrt!(root, "BIT_STRING".to_string()),
            the_boolean: the_boolean.clone(),
            the_any: nrt!(root, "ANY".to_string()),
            the_none: nrt!(root, "NONE".to_string()),
            the_map: nrt!(root, "MAP".to_string()),
            the_product: nrt!(root, "PRODUCT".to_string()),
            the_special_form: nrt!(root, "SPECIAL_FORM".to_string()),
            the_properties: nrt!(root, "PROPERTIES".to_string()),

            // Initialize the constants.
            is_true: Arc::new(Term::BooleanLiteral{
                locus: Locus::Internal, typ: the_boolean.clone(), value: true}),
            is_false: Arc::new(Term::BooleanLiteral{
                locus: Locus::Internal, typ: the_boolean.clone(), value: false}),

            // Initialize the term storage.
            named_terms: HashMap::new(),
        };
        fact.named_terms.insert("^ROOT".to_string(), fact.the_root.clone());
        fact.named_terms.insert("SYMBOL".to_string(), fact.the_symbol.clone());
        fact.named_terms.insert("STRING".to_string(), fact.the_string.clone());
        fact.named_terms.insert("INTEGER".to_string(), fact.the_integer.clone());
        fact.named_terms.insert("FLOAT".to_string(), fact.the_float.clone());
        fact.named_terms.insert("BIT_STRING".to_string(), fact.the_bit_string.clone());
        fact.named_terms.insert("BOOLEAN".to_string(), fact.the_boolean.clone());
        fact.named_terms.insert("ANY".to_string(), fact.the_any.clone());
        fact.named_terms.insert("NONE".to_string(), fact.the_none.clone());
        fact.named_terms.insert("MAP".to_string(), fact.the_map.clone());
        fact.named_terms.insert("PRODUCT".to_string(), fact.the_product.clone());
        fact.named_terms.insert("SPECIAL_FORM".to_string(), fact.the_special_form.clone());
        fact.named_terms.insert("PROPERTIES".to_string(), fact.the_properties.clone());
        fact
    }

    /// Get a named root term by its name.
    /// name: The name of the term.
    pub fn get_named_root_term(&self, name: &String) -> Option<Arc<Term>> {
        match &self.named_terms.get(name) {
            &Some(term) => Some(term.clone()),
            &None => None,
        }
    }

    /// Determine if a given name is a named root term.
    pub fn is_named_root_term(&self, name: &String) -> bool {
        self.named_terms.contains_key(name)
    }

    /// Get the unique root term.
    pub fn get_root(&self) -> Arc<Term> {
        self.the_root.clone()
    }

    /// Get the well-known string type.
    pub fn get_string(&self) -> Arc<Term> {
        self.the_string.clone()
    }

    /// Make a new string instance.
    pub fn new_string(&self, locus: Locus, value: String, typ: Arc<Term>) -> Arc<Term> {
        Arc::new(Term::StringLiteral{ locus: locus, typ: typ.clone(), value: value })
    }

    /// Get the well-known symbol type.
    pub fn get_symbol(&self) -> Arc<Term> {
        self.the_symbol.clone()
    }

    /// Make a new symbol instance.
    pub fn new_symbol(&self, locus: Locus, value: String, typ: Arc<Term>) -> Arc<Term> {
        Arc::new(Term::SymbolLiteral{ locus: locus, typ: typ.clone(), value: value })
    }

    /// Get the well-known Boolean type.
    pub fn get_boolean(&self) -> Arc<Term> {
        self.the_boolean.clone()
    }

    /// Make a new Boolean instance.
    pub fn new_boolean(&self, value: bool) -> Arc<Term> {
        if value { self.is_true.clone() }
        else { self.is_false.clone() }
    }

    /// Make a new variable.
    pub fn new_variable(&self, locus: Locus, typ: &Arc<Term>, name: String,
        guard: &Arc<Term>) -> Arc<Term> {
        Arc::new(
            Term::Variable {
                locus: locus,
                typ: typ.clone(),
                name: name,
                guard: guard.clone(),
            }
        )
    }

    /// Make a new static map.
    pub fn new_static_map(&self, locus: Locus, domain: &Arc<Term>,
        codomain: &Arc<Term>) -> Arc<Term> {
        Arc::new(
            Term::StaticMap {
                locus: locus,
                domain: domain.clone(),
                codomain: codomain.clone(),
            }
        )
    }

    /// Make a new static product.
    pub fn new_static_product(&self, locus: Locus, lhs: &Arc<Term>, rhs: &Arc<Term>) -> Arc<Term> {
        Arc::new(
            Term::StaticProduct {
                locus: locus,
                lhs: lhs.clone(),
                rhs: rhs.clone(),
            }
        )
    }

    /// Make a new lambda.
    pub fn new_lambda(&self, locus: Locus, param: &Arc<Term>, body: &Arc<Term>,
        guard: &Arc<Term>) -> Arc<Term> {
        Arc::new(
            Term::Lambda {
                locus: locus,
                param: param.clone(),
                body: body.clone(),
                guard: guard.clone(),
            }
        )
    }

    /// Get the type of the provided term.
    pub fn get_type(&self, term: &Arc<Term>) -> Arc<Term> {
        match **term {
            Term::Root => Arc::new(Term::Root),
            Term::SymbolLiteral { ref locus, ref typ, .. } => typ.clone(),
            Term::StringLiteral { ref locus, ref typ, .. } => typ.clone(),
            Term::BooleanLiteral { ref locus, ref typ, .. } => typ.clone(),
            Term::Variable { ref locus, ref typ, .. } => typ.clone(),
            Term::StaticMap { .. } => self.the_map.clone(),
            Term::StaticProduct { .. } => self.the_product.clone(),
            Term::Lambda { ref locus, ref param, ref body, .. } => {
                let lhs = self.get_type(param);
                let rhs = self.get_type(body);
                self.new_static_product(Locus::Internal, &lhs, &rhs)
            }
        }
    }

    pub fn get_locus(&self, term: &Arc<Term>) -> Arc<Locus> {
        match **term {
            Term::Root => Arc::new(Locus::Internal),
            Term::SymbolLiteral { ref locus, .. } => locus.clone(),
            Term::StringLiteral { ref locus, .. } => locus.clone(),
            Term::BooleanLiteral { ref locus, .. } => locus.clone(),
            Term::Variable { ref locus, .. } => locus.clone(),
            Term::StaticMap { ref locus, .. } => locus.clone(),
            Term::StaticProduct { ref locus, .. } => locus.clone(),
            Term::Lambda { ref locus, .. } => locus.clone(),
        }
    }
}
