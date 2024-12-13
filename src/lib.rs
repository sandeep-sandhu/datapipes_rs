//! # Simple multi-threaded Data pipelines in Rust
//!
//! Rust-native library for simplifying building and executing data pipelines in Rust.
//!
//!
//! ## Get Started
//! Get started using this crate in just a few lines of code, for example:
//!
//! <tt>
//! use std::env;<br/>
//! use datapipes;<br/>
//!<br/>
//! fn main() {<br/>
//!     <br/>
//!     }<br/>
//!<br/>
//!
//! Refer to the README file and the source code of these in the plugins folder and roll out your own plugins.
//! 


pub mod mt;
pub mod utils;
pub mod dataobj;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
const CARGO_PKG_NAME: &str = env!("CARGO_PKG_NAME");

