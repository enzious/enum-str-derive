//! # enum-str-derive
//!
//! [![License](https://img.shields.io/github/license/enzious/enum-str-derive)](https://github.com/enzious/enum-str-derive/blob/master/LICENSE.md)
//! [![Contributors](https://img.shields.io/github/contributors/enzious/enum-str-derive)](https://github.com/enzious/enum-str-derive/graphs/contributors)
//! [![GitHub Repo stars](https://img.shields.io/github/stars/enzious/enum-str-derive?style=social)](https://github.com/enzious/enum-str-derive)
//! [![crates.io](https://img.shields.io/crates/v/enum-str-derive.svg)](https://crates.io/crates/enum-str-derive)
//!
//! A crate to serialize/deserialize enums into/from a string.
//!
//! Converts enums to a string when using [serde] and [postgres].
//!
//! ## Documentation
//!
//! - [API Documentation](https://crates.io/crates/enum-str-derive)
//!
//! ## Implementation
//!
//! ```rust
//! use enum_str_derive::EnumStr;
//!
//! #[derive(Clone, Copy, Debug, EnumStr)]
//! pub enum ChannelTypeShortcode {
//!   Text, // TEXT
//!   #[enum_str(string = "w")]
//!   Theater, // w
//! }
//! ```
//!
//! [serde]: https://crates.io/crates/serde
//! [postgres]: https://crates.io/crates/postgres

use proc_macro::TokenStream;

mod enum_str;

#[proc_macro_derive(EnumStr, attributes(enum_str))]
pub fn derive_enum_str(input: TokenStream) -> TokenStream {
  enum_str::derive_enum_str(input)
}
