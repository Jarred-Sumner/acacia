//! A spatial tree library

#![feature(phase)]
#![feature(unboxed_closures)]


extern crate nalgebra;

#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[phase(plugin)]
extern crate quickcheck_macros;
#[cfg(test)]
#[phase(plugin)]
extern crate nalgebra;
#[cfg(test)]
extern crate test;

#[warn(missing_docs)]
pub mod ntree;
#[warn(missing_docs)]
pub mod tree;
#[warn(missing_docs)]
pub mod util;
