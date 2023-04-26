use std::collections::HashMap;

use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use quote::quote;
use syn::DeriveInput;

pub fn derive_enum_str(input: TokenStream) -> TokenStream {
  let ast = syn::parse_macro_input!(input as DeriveInput);

  let name = ast.ident;

  let variants = if let syn::Data::Enum(syn::DataEnum { variants, .. }) = ast.data {
    variants
  } else {
    unimplemented!();
  };

  let (case_map,) = variants
    .iter()
    .fold((HashMap::new(),), |(mut case_map,), variant| {
      let variant_ident = variant.ident.to_owned();

      let variant_cased = variant_ident.to_string().to_case(Case::ScreamingSnake);

      case_map.insert(variant_ident.to_owned(), variant_cased);

      for attr in &variant.attrs {
        if attr.path.segments.len() == 1 && attr.path.segments[0].ident == "enum_str" {
          let tokens = attr.tokens.to_owned();

          for token in tokens.into_iter() {
            if let TokenTree::Group(group) = token {
              let mut tokens = group.stream().into_iter().peekable();

              while tokens.peek().is_some() {
                if let Some(TokenTree::Ident(ident)) = tokens.next() {
                  let ident = ident.to_string();

                  if tokens
                    .next()
                    .map(|punct| punct.to_string())
                    .filter(|punct| &punct as &str == "=")
                    .is_none()
                  {
                    panic!("Invalid #[enum_str] options");
                  }

                  match &ident as &str {
                    "string" => {
                      match tokens
                        .next()
                        .and_then(|token| match token {
                          TokenTree::Literal(literal) => Some(literal),
                          _ => None,
                        })
                        .and_then(|literal| {
                          let litty = syn::Lit::new(literal.to_owned());

                          match litty {
                            syn::Lit::Str(litty) => Some(litty.value()),
                            _ => None,
                          }
                        }) {
                        Some(literal) => {
                          case_map.insert(variant_ident.to_owned(), literal.into());
                        }
                        _ => panic!("Invalid #[enum_str(string)]"),
                      }
                    }

                    _ => {
                      panic!("Unknown #[enum_str] option: {}", &ident);
                    }
                  }
                }

                match tokens.next().and_then(|token| match token {
                  TokenTree::Punct(punct) => Some(punct.as_char()),
                  _ => None,
                }) {
                  Some(',') | None => {}
                  Some(token) => panic!("Invalid #[enum_str] options: at token {:?}", &token),
                }
              }
            }
          }
        }
      }

      (case_map,)
    });

  let to_str_cases = proc_macro2::TokenStream::from_iter(case_map.iter().map(|(ident, string)| {
    quote! {
      #name::#ident => #string,
    }
  }));

  let from_str_cases =
    proc_macro2::TokenStream::from_iter(case_map.iter().map(|(ident, string)| {
      quote! {
        #string => Ok(#name::#ident),
      }
    }));

  #[cfg(not(feature = "postgres"))]
  let postgres = quote! {};

  #[cfg(feature = "postgres")]
  let postgres = quote! {
    impl<'a> ::postgres_types::FromSql<'a> for #name {
      fn from_sql(
        ty: &::postgres_types::Type,
        raw: &[u8]
      ) -> Result<Self, Box<dyn std::error::Error + 'static + Sync + Send>> {
        let str: String = <String as ::postgres_types::FromSql>::from_sql(ty, raw)?;

        str.parse().map_err(|_| "".into())
      }

      fn accepts(ty: &::postgres_types::Type) -> bool {
        <String as ::postgres_types::FromSql>::accepts(ty)
      }
    }

    impl ::postgres_types::ToSql for #name {
      fn to_sql(
        &self,
        _: &::postgres_types::Type,
        w: &mut bytes::BytesMut
      ) -> Result<::postgres_types::IsNull, Box<dyn std::error::Error + 'static + Send + Sync>> {
        ::postgres_protocol::types::text_to_sql(self.to_str(), w);

        Ok(::postgres_types::IsNull::No)
      }

      fn accepts(ty: &::postgres_types::Type) -> bool {
        <String as ::postgres_types::ToSql>::accepts(ty)
      }

      ::postgres_types::to_sql_checked!();
    }
  };

  #[cfg(not(feature = "serde"))]
  let serde = quote! {};

  #[cfg(feature = "serde")]
  let serde = quote! {
    impl<'d> ::serde::Deserialize<'d> for #name {
      fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: ::serde::Deserializer<'d>,
      {
        struct Visitor;

        impl<'v> ::serde::de::Visitor<'v> for Visitor {
          type Value = #name;

          fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            write!(formatter, "a string for {}", stringify!(#name))
          }

          fn visit_str<E>(self, value: &str) -> Result<#name, E>
            where E: ::serde::de::Error,
          {
            match value.parse() {
              Ok(value) => Ok(value),
              _ => Err(E::invalid_value(::serde::de::Unexpected::Other(
                &format!("unknown {} variant: {}", stringify!(#name), value)
              ), &self)),
            }
          }
        }

        // Deserialize the enum from a string.
        deserializer.deserialize_str(Visitor)
      }
    }
  };

  let expanded = quote! {
    impl #name {
      pub fn to_str(&self) -> &'static str {
        match *self {
          #to_str_cases
        }
      }
    }

    impl std::str::FromStr for #name {
      type Err = ();
      fn from_str(name: &str) -> Result<Self, Self::Err> {
        match name {
          #from_str_cases
          _ => Err(()),
        }
      }
    }

    impl std::fmt::Display for #name {
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
      }
    }

    impl ::serde::Serialize for #name {
      fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: ::serde::Serializer,
      {
        serializer.serialize_str(self.to_str())
      }
    }

    #serde

    #postgres
  };

  TokenStream::from(expanded)
}
