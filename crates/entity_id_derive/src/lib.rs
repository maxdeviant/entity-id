use proc_macro::TokenStream;
use quote::quote;
use syn::{self, parse_macro_input, DeriveInput, Token};

use symbols::*;

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
                Self(ulid::Ulid::new())
            }

            pub fn unprefixed(&self) -> String {
                self.0.to_string().to_lowercase()
            }
        }

        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}_{}", #prefix, self.0.to_string().to_lowercase())
            }
        }

        impl From<uuid::Uuid> for #name {
            fn from(value: uuid::Uuid) -> Self {
                Self(ulid::Ulid::from(value))
            }
        }

        impl From<#name> for uuid::Uuid {
            fn from(value: #name) -> Self {
                Self::from(value.0)
            }
        }

        impl std::str::FromStr for #name {
            type Err = ulid::DecodeError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                fn unprefix_id(value: &str) -> &str {
                    value.split('_').last().to_owned().unwrap_or(value)
                }

                let value = unprefix_id(&s);

                Ok(Self(ulid::Ulid::from_string(value)?))
            }
        }

        impl TryFrom<String> for #name {
            type Error = ulid::DecodeError;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                fn unprefix_id(value: &str) -> &str {
                    value.split('_').last().to_owned().unwrap_or(value)
                }

                let value = unprefix_id(&value);

                Ok(Self(ulid::Ulid::from_string(value)?))
            }
        }

        impl TryFrom<&str> for #name {
            type Error = ulid::DecodeError;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                fn unprefix_id(value: &str) -> &str {
                    value.split('_').last().to_owned().unwrap_or(value)
                }

                let value = unprefix_id(value);

                Ok(Self(ulid::Ulid::from_string(value)?))
            }
        }
    };

    expanded.into()
}
