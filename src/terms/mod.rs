// Copyright (c) 2015 by Stacy Prowell.  All rights reserved.
//
// Licensed under the BSD 2-Clause license.  See the file LICENSE
// that is part of this distribution.  This file may not be copied,
// modified, or distributed except according to those terms.extern

//! Provide basic definitions of terms.

use std::collections::BitVec;
use std::rc::Rc;

/// Represents root terms.
struct Root ( () );
struct Any ( () );
struct ENone ( () );

#[derive(Debug)]
struct EString( String ) ;
impl EString{
    fn new(val: &str) -> Self { EString(val.to_string()) }
}

struct Symbol( String );
#[derive(Debug)]
struct Integer( isize );
impl Integer{
    fn new(val: isize) -> Self { Integer(val) }
}

struct Float( f64 );
struct BitString( BitVec );
struct Boolean( bool );

//For simplicity while experimenting, I'm not using Rc<T>
//Can't derive Debug automatically for Term
//#[derive(Debug)]
struct Lambda{
    parameter: Box<Term>,
    guard    : Box<Term>,
    body     : Box<Term>,
    //type,
    index    : usize,
    stuff     : (),
}
impl Lambda{
    fn new(p: Box<Term>, g: Box<Term>, b: Box<Term>) -> Self{
        Lambda{parameter: p, guard: g, body: b, index: 0, stuff: ()}
    }
}

trait RootType<T>{
    fn native(&self) -> &T;
}
impl RootType<String> for EString{
    fn native(&self) -> &String { &self.0 }
}
impl RootType<isize> for Integer{
    fn native(&self) -> &isize { &self.0 }
}



trait Term{
    fn to_string(&self) -> String { "This is a Term.".to_string() }
}

impl Term for EString{
    fn to_string(&self) -> String { "This is an EString".to_string() }   
}

impl Term for Integer{
    fn to_string(&self) -> String {"This is an Integer".to_string() }
}

impl Term for Lambda{
    fn to_string(&self) -> String {"This is a Lambda.".to_string() }
}

#[test]
fn term_type_check_test() -> (){
    let estring = EString("Test".to_string());
    let integer = Integer::new(3);
    let i2 = Integer::new(5);
    let lambda = Lambda::new(Box::new(estring), Box::new(integer),
                 Box::new(i2));
    // Uncomment the following line to see the term type checker assist in action
    // stringterm = integerterm;
    // The following line shoudl fail to compile because i32 does not have the Termable trait.
    //estring = integer;
    
    panic!("Intentionial panic. {:?}", lambda.to_string() );
}

