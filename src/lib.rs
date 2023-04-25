use proc_macro::TokenStream;

mod enum_str;

#[proc_macro_derive(EnumStr, attributes(enum_str))]
pub fn derive_enum_str(input: TokenStream) -> TokenStream {
  enum_str::derive_enum_str(input)
}
