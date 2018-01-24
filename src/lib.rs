#[cfg(feature="python27")]
extern crate python27_sys as pyffi;

#[cfg(feature="python3")]
extern crate python3_sys as pyffi;
extern crate cpython;
extern crate libc;
extern crate num_complex;
extern crate ndarray;

pub mod types;
pub mod array;
pub mod npyffi;
pub mod error;
pub mod convert;

pub use array::PyArray;
pub use types::*;
pub use error::*;
pub use convert::{IntoPyArray, ToPyArray};
pub use npyffi::{PyArrayModule, PyUFuncModule};
