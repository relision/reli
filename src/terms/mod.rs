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
#[derive(Debug)]
struct Term<T> ( T );

trait Termable<T>{
    fn new(value: T) -> Self;
    fn native(&self) -> &T;
}

impl Termable<String> for EString{
    fn new(value: String) -> EString{
        EString(value)
    }
    fn native(&self) -> &String {
        &self.0
    }
}

impl Termable<isize> for Integer{
    fn new(value: isize) -> Integer{
        Integer(value)
    }
    fn native(&self) -> &isize {
        &self.0
    }
}

impl<T> Termable<T> for Term<T>{
    fn new(t: T) -> Term<T> {
        Term ( t )
    }
    fn native(&self) -> &T { &self.0 }
}

impl<T> Term<T>
{
    pub fn new<U>(t: U) -> Term<U> where U: Termable<T> {
        Term ( t )
    }
    pub fn unwrap(&self) -> &T {
        &self.0
    }
}

#[test]
fn term_type_check_test() -> (){
    let termable = EString::new("Test".to_string());
    let mut stringterm = Term::new(termable);
    let integerterm = Term::new(Integer(3));
    // Uncomment the following line to see the term type checker assist in action
    // stringterm = integerterm;
    // The following line shoudl fail to compile because i32 does not have the Termable trait.
    // let failterm = Term::new(3);
    
    //Nested terms don't work right now
    let nestedterm = Term::new(Term::new(Integer(3)));
    let stringval = &stringterm.unwrap().native();
    panic!("Intentionial panic. {:?}", stringval);
}

