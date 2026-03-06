//@ compile-flags: --emit=link

// Regression test for https://github.com/rust-lang/rust/issues/152653

#![feature(min_generic_const_args)]
#![allow(incomplete_features)]

type const CONST: usize = 1_i32;
//~^ ERROR the constant `1` is not of type `usize`

fn main() {
    CONST;
}
