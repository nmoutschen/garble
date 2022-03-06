#![warn(missing_debug_implementations, missing_docs, unreachable_pub)]
#![cfg_attr(docsrs, feature(doc_cfg))]
//! Data garbling crate
//!
//! The purpose of this crate is to provide a way to slightly modify data in
//! controlled way for fault injection purposes.
//!
//! ## Example
//!
//! ```rust
//! use garble::{Garble, SimpleGarbler};
//!
//! // Create a garbler with a 50% probability of garbling data
//! let mut garbler = SimpleGarbler::new(0.5);
//!
//! // Garble some data
//! dbg!(true.garble(&mut garbler));
//! dbg!(128u64.garble(&mut garbler));
//! dbg!((3.5_f32).garble(&mut garbler));
//! ```
//!
//! ## Derive macro
//!
//! This crate provides a derive macro for garbling structs.
//!
//! ```rust
//! use garble::{Garble, SimpleGarbler};
//!
//! #[derive(Debug, Garble)]
//! struct MyStruct {
//!     a: u32,
//! }
//!
//! // Create a garbler with a 50% probability of garbling data
//! let mut garbler = SimpleGarbler::new(0.5);
//!
//! // Garble some data
//! dbg!(MyStruct { a: 128 }.garble(&mut garbler));
//! ```
//! 
//! ### Skip garbling fields
//! 
//! You can use the `#[nogarble]` attribute to skip garbling specific fields:
//! 
//! ```rust
//! use garble::{Garble, SimpleGarbler};
//!
//! #[derive(Debug, Garble)]
//! struct MyStruct {
//!     a: u32,
//!     #[nogarble]
//!     b: u32,
//! }
//!
//! // Create a garbler with a 50% probability of garbling data
//! let mut garbler = SimpleGarbler::new(0.5);
//!
//! // Garble some data
//! dbg!(MyStruct { a: 128, b: 127 }.garble(&mut garbler));
//! ```
//!

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
