// Copyright (c) 2015 by Stacy Prowell.  All rights reserved.
//
// Licensed under the BSD 2-Clause license.  See the file LICENSE
// that is part of this distribution.  This file may not be copied,
// modified, or distributed except according to those terms.extern

//! Provide basic definitions of terms.

use std::collections::BitVec;
//use std::tuple::Tuple1;

/// Represents root terms.
struct Root ( () );
struct Any ( () );
struct ENone ( () );
#[derive(Debug)]
struct EString( String ) ;
struct Symbol( String );
#[derive(Debug)]
struct Integer( isize );
struct Float( f64 );
struct BitString( BitVec );
struct Boolean( bool );

trait Termable<T>{
    fn new(value: T) -> Self;
    fn native(&self) -> &T;
}

impl Termable<String> for EString{
    fn new(value: String) -> Self{
        EString(value)
    }
    fn native(&self) -> &String {
        &self.0
    }
}

#[derive(Debug)]
struct Term<T> {
    value: T,
}

impl<T> Term<T>
{
    pub fn new(t: T) -> Term<T> {
         
        Term { value: t }
    }
    pub fn unwrap(&self) -> &T {
        &self.value
    }
}

#[test]
fn term_type_check_test() -> (){
    let stringterm = Term::new(EString::new("test".to_string()));
    let nestedterm = Term::new(Term::new(Integer(3)));
    //how to make this access not require .0?
    let stringval = &stringterm.unwrap().native();
    panic!("Intentionial panic. {:?}", stringval);
}

