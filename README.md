# enum-str-derive

[![License](https://img.shields.io/github/license/enzious/enum-str-derive)](https://github.com/enzious/enum-str-derive/blob/master/LICENSE.md)
[![Contributors](https://img.shields.io/github/contributors/enzious/enum-str-derive)](https://github.com/enzious/enum-str-derive/graphs/contributors)
[![GitHub Repo stars](https://img.shields.io/github/stars/enzious/enum-str-derive?style=social)](https://github.com/enzious/enum-str-derive)
[![crates.io](https://img.shields.io/crates/v/enum-str-derive.svg)](https://crates.io/crates/enum-str-derive)

A crate to serialize/deserialize enums into/from a string.

## Documentation

- [API Documentation](https://crates.io/crates/enum-str-derive)

## Implementation

```rust
#[derive(Clone, Copy, Debug, EnumStr)]
pub enum ChannelTypeShortcode {
  #[enum_str(string = "t")]
  Text,
  #[enum_str(string = "w")]
  Theater,
}
```
