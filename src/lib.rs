//! # shell-parser-descript-rs
//!
//! Parse descript.txt for shell settings on Ukagaka.
//!
//! ## Example
//!
//! ```
//! use std::{fs::File, io::Read, path::PathBuf};
//! use shell_parser_descript_rs::{decode_bytes, parse};
//!
//! let file_path =
//!     PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_target/descript/descript.txt");
//! let mut file = match File::open(file_path) {
//!     Ok(v) => v,
//!     Err(e) => {
//!         eprintln!("{:?}", e);
//!         return;
//!     }
//! };
//! let mut buffer = Vec::new();
//!
//! if let Err(e) = file.read_to_end(&mut buffer) {
//!     eprintln!("{:?}", e);
//!     return;
//! };
//!
//! let content = match decode_bytes(&buffer) {
//!     Ok(v) => v,
//!     Err(e) => {
//!         eprintln!("{:?}", e);
//!         return;
//!     }
//! };
//!
//! let shell_descript = match parse(&content) {
//!     Ok(v) => v,
//!     Err(e) => {
//!         eprintln!("{:?}", e);
//!         return;
//!     }
//! };
//! assert!(!shell_descript.lines().is_empty());
//! ```

pub mod ast;
pub mod parse;

pub use ast::*;
pub use parse::*;
