use std::fmt::Display;

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, parse_macro_input, DeriveInput, Token};
use ulid::Ulid;
use uuid::Uuid;

use symbols::*;

fn unprefix_id(value: &str) -> &str {
    value.split('_').last().to_owned().unwrap_or(value)
}

mod symbols {
    use std::fmt::{self, Display};
    use syn::{Ident, Path};

    #[derive(Debug, Clone, Copy)]
    pub struct Symbol(&'static str);

    impl PartialEq<Symbol> for Ident {
        fn eq(&self, word: &Symbol) -> bool {
            self == word.0
        }
    }

    impl<'a> PartialEq<Symbol> for &'a Ident {
        fn eq(&self, word: &Symbol) -> bool {
            *self == word.0
        }
    }

    impl PartialEq<Symbol> for Path {
        fn eq(&self, word: &Symbol) -> bool {
            self.is_ident(word.0)
        }
    }

    impl<'a> PartialEq<Symbol> for &'a Path {
        fn eq(&self, word: &Symbol) -> bool {
            self.is_ident(word.0)
        }
    }

    impl Display for Symbol {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str(self.0)
        }
    }

    pub const ENTITY_ID: Symbol = Symbol("entity_id");

    pub const PREFIX: Symbol = Symbol("prefix");
}

#[proc_macro_derive(EntityId, attributes(entity_id))]
pub fn entity_id(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let mut prefix = None;

    for attr in &input.attrs {
        if attr.path() != ENTITY_ID {
            continue;
        }

        attr.parse_nested_meta(|meta| {
            // #[entity_id(prefix = "foo")]
            if meta.path == PREFIX {
                let lookahead = meta.input.lookahead1();
                if lookahead.peek(Token![=]) {
                    use syn::{Expr, ExprLit, Lit};

                    let expr: Expr = meta.value()?.parse()?;

                    let mut value = &expr;
                    while let Expr::Group(group) = value {
                        value = &group.expr;
                    }

                    if let Expr::Lit(ExprLit {
                        lit: Lit::Str(lit), ..
                    }) = value
                    {
                        prefix = Some(lit.value())
                    } else {
                        return Err(meta.error(format!(
                            "expected `{}` attribute to be a string: `{} = \"...\"`",
                            PREFIX, PREFIX,
                        )));
                    }
                }

                return Ok(());
            }

            Err(meta.error(format!("unrecognized {}", ENTITY_ID)))
        })
        .expect("failed to parse attribute meta");
    }

    let prefix = prefix.unwrap_or("entity".to_string());

    let expanded = quote! {
        impl #name {
            pub fn new() -> Self {
                Self(Ulid::new())
            }
        }

        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}_{}", #prefix, self.0.to_string().to_lowercase())
            }
        }
    };

    expanded.into()
}

// #[macro_export]
// macro_rules! entity_id {
//     ($entity_id:ty, $prefix:expr) => {
//         impl $entity_id {
//             pub fn new() -> Self {
//                 Self(Ulid::new())
//             }

//             pub fn unprefixed(&self) -> String {
//                 self.0.to_string().to_lowercase()
//             }
//         }

//         impl Display for $entity_id {
//             fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//                 write!(f, "{}_{}", $prefix, self.0.to_string().to_lowercase())
//             }
//         }

//         impl From<Uuid> for $entity_id {
//             fn from(value: Uuid) -> Self {
//                 Self(Ulid::from(value))
//             }
//         }

//         impl From<$entity_id> for Uuid {
//             fn from(value: $entity_id) -> Self {
//                 Self::from(value.0)
//             }
//         }

//         impl FromStr for $entity_id {
//             type Err = ulid::DecodeError;

//             fn from_str(s: &str) -> Result<Self, Self::Err> {
//                 let value = crate::unprefix_id(&s);

//                 Ok(Self(Ulid::from_string(value)?))
//             }
//         }

//         impl TryFrom<String> for $entity_id {
//             type Error = ulid::DecodeError;

//             fn try_from(value: String) -> Result<Self, Self::Error> {
//                 let value = crate::unprefix_id(&value);

//                 Ok(Self(Ulid::from_string(value)?))
//             }
//         }

//         impl TryFrom<&str> for $entity_id {
//             type Error = ulid::DecodeError;

//             fn try_from(value: &str) -> Result<Self, Self::Error> {
//                 let value = crate::unprefix_id(value);

//                 Ok(Self(Ulid::from_string(value)?))
//             }
//         }
//     };
// }

// #[cfg(test)]
// mod tests {
//     use std::str::FromStr;

//     use super::*;

//     // #[derive(EntityId)]
//     struct UserId(Ulid);

//     #[test]
//     fn new_generates_an_id_with_the_given_prefix() {
//         let user_id = UserId::new();

//         assert!(user_id.to_string().starts_with("user_"));
//     }

//     #[test]
//     fn unprefixed_returns_the_id_without_the_prefix() {
//         let user_id = UserId::new();

//         assert_eq!(user_id.unprefixed(), user_id.0.to_string().to_lowercase());
//     }

//     #[test]
//     fn entity_id_from_uuid() {
//         let uuid = Uuid::from_str("14a20d59-4d68-4bdf-aac6-8e1af037d183").unwrap();

//         let user_id = UserId::from(uuid);

//         assert_eq!(user_id.to_string(), "user_0mm86njkb89fftnhme3br3fmc3");
//     }

//     #[test]
//     fn uuid_from_entity_id() {
//         let user_id = UserId::from_str("user_2wdncp35529bet2md0kzxrj0bs").unwrap();

//         let uuid = Uuid::from(user_id);

//         assert_eq!(
//             uuid,
//             Uuid::from_str("5c6d5961-94a2-4add-a151-a09ffb890179").unwrap()
//         )
//     }
// }
