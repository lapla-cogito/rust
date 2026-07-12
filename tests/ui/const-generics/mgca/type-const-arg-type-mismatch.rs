// Regression test for https://github.com/rust-lang/rust/issues/154805

#![feature(min_generic_const_args)]
#![feature(generic_const_exprs)]
#![feature(generic_const_items)]

type const ADD1<const N: usize>: usize = const { N + 1 };
//~^ ERROR unconstrained generic constant

type const ONE: usize = ADD1::<b"">;
//~^ ERROR the constant `*b""` is not of type `usize`

fn main() {}
