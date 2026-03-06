//@ compile-flags: -Zvalidate-mir

// Regression test for https://github.com/rust-lang/rust/issues/152962

#![feature(min_generic_const_args)]
#![allow(incomplete_features)]

pub struct A;

pub trait Array {
    type const LEN: usize;
    fn arr() -> [u8; Self::LEN];
}

impl Array for A {
    type const LEN: usize = 0u8;
    //~^ ERROR the constant `0` is not of type `usize`

    fn arr() -> [u8; const { Self::LEN }] {}
}

fn main() {}
