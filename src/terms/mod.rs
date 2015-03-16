// Copyright (c) 2015 by Stacy Prowell.  All rights reserved.
//
// Licensed under the BSD 2-Clause license.  See the file LICENSE
// that is part of this distribution.  This file may not be copied,
// modified, or distributed except according to those terms.extern

//! Provide basic definitions of terms.

/// Represent an instance of a term.
#[derive(Debug, Clone)]
pub enum TermKind {
	ERoot,
	EAny,
	ENone,
	Symbol,//(Box<Term>, String),
	EString,//(Box<Term>, String),
	Integer,//(Box<Term>, i64),
	Float,//(Box<Term>, f64),
	BitString,//(Box<Term>, Vec<i8>),
	Boolean,//(Box<Term>, bool),
}

/*pub trait Term {
    //Return the kind
     fn get_kind(&self) -> TermKind;
}*/

struct ERoot ( pub () );
struct EAny ( pub () );
struct ENone ( pub () );
#[derive(Debug)]
struct EString(pub String);
struct Symbol(pub String);
#[derive(Debug)]
struct Integer(pub isize);
struct Float(pub f64);
struct BitString(pub Vec<i8>);
struct Boolean(pub bool);

#[derive(Debug)]
struct Term<T> {
    value: T,
}

impl<T> Term<T> {
    pub fn new(t: T) -> Term<T> {
        Term { value: t }
    }
}

/*impl Term for RootTerm{
    fn get_kind(&self) -> TermKind { self.kind.clone() }
}*/

#[test]
fn term_type_check_test() -> (){
    let stringterm = Term::new(EString("test".to_string()));
    let nestedterm = Term::new(Term::new(Integer(3)));
    
    //let rootkind = rootterm.get_kind();
    //println!("{:?}", rootkind);
    panic!("Intential fail. {:?}", nestedterm);
}

