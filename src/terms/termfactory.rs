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
     * The known root terms.  We just want one instance of each, and so this struct holds
     * the instances.  This avoids the lookup cost when constructing new instances with the
     * default type.
     */

    /// Root.
    the_root: Arc<Term>,
    /// SYMBOL type.
    the_symbol: Arc<Term>,
    /// STRING type.
    the_string: Arc<Term>,
    /// INTEGER type.
    #[allow(dead_code)]
    the_integer: Arc<Term>,
    /// FLOAT type.
    #[allow(dead_code)]
    the_float: Arc<Term>,
    /// BIT_STRING type.
    #[allow(dead_code)]
    the_bit_string: Arc<Term>,
    /// BOOLEAN type.
    the_boolean: Arc<Term>,
    /// ANY type.
    #[allow(dead_code)]
    the_any: Arc<Term>,
    /// NONE type.
    #[allow(dead_code)]
    the_none: Arc<Term>,
    /// MAP type.
    the_map: Arc<Term>,
    /// PRODUCT type.
    the_product: Arc<Term>,
    /// SPECIAL_FORM type.
    #[allow(dead_code)]
    the_special_form: Arc<Term>,
    /// PROPERTIES type.
    #[allow(dead_code)]
    the_properties: Arc<Term>,

    /*
     * Well-known constants.  We just want one instance of each, and so this struct holds
     * the instances.
     */

    /// The Boolean true.
    is_true: Arc<Term>,
    /// The Boolean false.
    is_false: Arc<Term>,

    /*
     * Store a mapping from the name of a named root term to the root term itself.
     */

    /// Map names to root terms.
    named_terms: HashMap<String, Arc<Term>>
}

/// Local macro to create a new root term.
macro_rules! nrt {
    ($typ: expr, $name: expr) => {
        Arc::new(Term::SymbolLiteral {
            locus: Locus::Internal,         // Always internal.
            typ: $typ.clone(),              // Clone the type so we can store it.
            value: $name                    // The name of the root term.
        })
    }
}

impl TermFactory {
    /// Construct and return a new term factory.
    pub fn new() -> Self {
        // Make all the terms.
        let root = Arc::new(Term::Root);
        let symbol = nrt!(root, "SYMBOL".to_string());
        let string = nrt!(root, "STRING".to_string());
        let integer = nrt!(root, "INTEGER".to_string());
        let float = nrt!(root, "FLOAT".to_string());
        let bit_string = nrt!(root, "BIT_STRING".to_string());
        let boolean = nrt!(root, "BOOLEAN".to_string());
        let any = nrt!(root, "ANY".to_string());
        let none = nrt!(root, "NONE".to_string());
        let map = nrt!(root, "MAP".to_string());
        let product = nrt!(root, "PRODUCT".to_string());
        let special_form = nrt!(root, "SPECIAL_FORM".to_string());
        let properties = nrt!(root, "PROPERTIES".to_string());

        // Make the hash map now.  We will store it in the term factory when we
        // construct it.
        let mut hmap = HashMap::new();
        hmap.insert("ROOT".to_string(), root.clone());
        hmap.insert("SYMBOL".to_string(), symbol.clone());
        hmap.insert("STRING".to_string(), string.clone());
        hmap.insert("INTEGER".to_string(), integer.clone());
        hmap.insert("FLOAT".to_string(), float.clone());
        hmap.insert("BIT_STRING".to_string(), bit_string.clone());
        hmap.insert("BOOLEAN".to_string(), boolean.clone());
        hmap.insert("ANY".to_string(), any.clone());
        hmap.insert("NONE".to_string(), none.clone());
        hmap.insert("MAP".to_string(), map.clone());
        hmap.insert("PRODUCT".to_string(), product.clone());
        hmap.insert("SPECIAL_FORM".to_string(), special_form.clone());
        hmap.insert("PROPERTIES".to_string(), properties.clone());

        // Create the term factory instance.  This initializes the root terms.
        let fact = TermFactory {
            // Initialize the Boolean constants.
            is_true: Arc::new(Term::BooleanLiteral{
                locus: Locus::Internal, typ: boolean.clone(), value: true}),
            is_false: Arc::new(Term::BooleanLiteral{
                locus: Locus::Internal, typ: boolean.clone(), value: false}),

            // Initialize the root terms.  Note that we clone the terms we have already
            // created when storing them, since our copy will be deallocated when it goes
            // out of scope.  Also, note that we built the clone for root into the macro.
            the_root: root,
            the_symbol: symbol,
            the_string: string,
            the_integer: integer,
            the_float: float,
            the_bit_string: bit_string,
            the_boolean: boolean,
            the_any: any,
            the_none: none,
            the_map: map,
            the_product: product,
            the_special_form: special_form,
            the_properties: properties,

            // Save the map.
            named_terms: hmap
        };

        // The factory is done.
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

    /// Get the any term.
    pub fn get_any(&self) -> Arc<Term> {
        self.the_any.clone()
    }

    /// Get the none term.
    pub fn get_none(&self) -> Arc<Term> {
        self.the_none.clone()
    }

    /// Get the well-known string type.
    pub fn get_string(&self) -> Arc<Term> {
        self.the_string.clone()
    }

    /// Make a new string instance.
    pub fn new_string(&self, locus: Locus, value: String) -> Arc<Term> {
        Arc::new(Term::StringLiteral{ locus: locus, typ: self.the_string.clone(), value: value })
    }

    /// Make a typed string instance.
    pub fn new_typed_string(&self, locus: Locus, value: String, typ: Arc<Term>) -> Arc<Term> {
        Arc::new(Term::StringLiteral{ locus: locus, typ: typ.clone(), value: value })
    }

    /// Get the well-known symbol type.
    pub fn get_symbol(&self) -> Arc<Term> {
        self.the_symbol.clone()
    }

    /// Make a new symbol instance.
    pub fn new_symbol(&self, locus: Locus, value: String) -> Arc<Term> {
        Arc::new(Term::SymbolLiteral{ locus: locus, typ: self.the_symbol.clone(), value: value })
    }

    /// Make a typed symbol instance.
    pub fn new_typed_symbol(&self, locus: Locus, value: String, typ: Arc<Term>) -> Arc<Term> {
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

    /// Make a typed Boolean instance.
    pub fn new_typed_boolean(&self, locus: Locus, value: bool, typ: Arc<Term>) -> Arc<Term> {
        Arc::new(Term::BooleanLiteral{ locus: locus, typ: typ.clone(), value: value })
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
            Term::SymbolLiteral { ref typ, .. } => typ.clone(),
            Term::StringLiteral { ref typ, .. } => typ.clone(),
            Term::BooleanLiteral { ref typ, .. } => typ.clone(),
            Term::Variable { ref typ, .. } => typ.clone(),
            Term::StaticMap { .. } => self.the_map.clone(),
            Term::StaticProduct { .. } => self.the_product.clone(),
            Term::Lambda { ref param, ref body, .. } => {
                let lhs = self.get_type(param);
                let rhs = self.get_type(body);
                self.new_static_product(Locus::Internal, &lhs, &rhs)
            }
        }
    }

    /// Get the locus of the provided term.
    pub fn get_locus(&self, term: &Arc<Term>) -> Locus {
        match **term {
            Term::Root => Locus::Internal.clone(),
            Term::SymbolLiteral { ref locus, .. } => locus.clone(),
            Term::StringLiteral { ref locus, .. } => locus.clone(),
            Term::BooleanLiteral { ref locus, .. } => locus.clone(),
            Term::Variable { ref locus, .. } => locus.clone(),
            Term::StaticMap { ref locus, .. } => locus.clone(),
            Term::StaticProduct { ref locus, .. } => locus.clone(),
            Term::Lambda { ref locus, .. } => locus.clone()
        }
    }
}
