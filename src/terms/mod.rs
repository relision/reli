
/*
 * This is a block comment.
 * /*
 *  * This is a nested block comment.
 *  * These are permitted!
 *  */
 * More outer comment.
 */
fn main() {
	println!(r##"Hello, "world."  With a \n at the end."##);
}

/**
 * The enumerated type for terms.
 */
pub enum Term {
	Root,
	Any,
	None,
	Symbol(Box<Term>, String),
	String(Box<Term>, String),
	Integer(Box<Term>, i64),
	Float(Box<Term>, f64),
	BitString(Box<Term>, i64, i64),
	Boolean(Box<Term>, bool),
}