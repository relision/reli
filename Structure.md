# Structure

This file outlines the basic structure of the relision library.

## termfactory

The `termfactory` module contains the `TermFactory` struct that can create
terms.  In order for this to work, the factory has to know `^ROOT`, which is
special.

The basic literals are:

  * integers
  * floating point numbers
  * strings
  * symbols
  * bit strings
  * Booleans

The `TermFactory` instance does not know anything about default types for these
terms.  This complicates the use of the factory (since now the caller must know
the default types), but it means the factory does not have to be bootstrapped.

Bootstrapping requires that we create the most primitive "root types."  For
instance we create `SYMBOL` as a symbol with type `^ROOT`.

```rust
// Let's create a "normal" symbol.
val fact = TermFactory::new();
val symbol = fact.new_symbol(INTERNAL, "SYMBOL", fact.the_root);
val normal = fact.new_symbol(INTERNAL, "normal", symbol);
```

This code creates the root type `SYMBOL` and then uses it to create a normal
symbol.

The same logic applies for the Boolean literals.

```rust
// Let's create the Boolean literals.
val fact = TermFactory::new();
val boolean = fact.new_symbol(INTERNAL, "BOOLEAN", fact.the_root);
val true = fact.new_boolean(INTERNAL, true, boolean);
val false = fact.new_boolean(INTERNAL, false, boolean);
```

The `INTERNAL` argument specifies the locus of the term.  Every term has an
associated locus, which specifies the place where the term originates.  This
can be a line in a file or on the console, or it can be `INTERNAL`.

## Arc

Every term is delivered as an `Arc<Term>`, or an atomically reference counted
object.  Terms are cloned when stored.  Loci do not need to be cloned, since
a given locus (with the exception of `INTERNAL`) is unique to the term.

## Writing and Reading
