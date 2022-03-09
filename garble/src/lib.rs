#![warn(missing_debug_implementations, missing_docs, unreachable_pub)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../../README.md")]

mod garble;
pub use crate::garble::Garble;

mod impls;

mod garbler;
pub use crate::garbler::Garbler;
#[cfg(feature = "simple")]
#[cfg_attr(docsrs, doc(cfg(feature = "simple")))]
pub use crate::garbler::SimpleGarbler;

#[cfg(feature = "derive")]
#[allow(unused_imports)]
#[macro_use]
extern crate garble_derive;
pub use garble_derive::*;
