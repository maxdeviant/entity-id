mod symbols;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{self, DeriveInput, Token};

use symbols::*;

pub fn expand_derive_entity_id(
    input: &mut DeriveInput,
) -> Result<proc_macro2::TokenStream, Vec<syn::Error>> {
    let name = &input.ident;

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

            let path = meta.path.to_token_stream().to_string().replace(' ', "");
            Err(meta.error(format_args!(
                "unknown `{}` attribute: `{}`",
                ENTITY_ID, path
            )))
        })
        .map_err(|err| vec![err])?;
    }

    let prefix = prefix.unwrap_or("entity".to_string());

    let prefix_doc_string = format!("The prefix used for a [`{}`].", name);
    let new_doc_string = format!("Returns a new [`{}`].", name);

    let uuid_impls = if cfg!(feature = "uuid") {
        quote! {
            #[automatically_derived]
            impl From<uuid::Uuid> for #name {
                fn from(value: uuid::Uuid) -> Self {
                    Self(ulid::Ulid::from(value))
                }
            }

            #[automatically_derived]
            impl From<#name> for uuid::Uuid {
                fn from(value: #name) -> Self {
                    Self::from(value.0)
                }
            }
        }
    } else {
        TokenStream::new()
    };

    let expanded = quote! {
        impl #name {
            #[doc = #prefix_doc_string]
            pub const PREFIX: &'static str = #prefix;

            #[doc = #new_doc_string]
            pub fn new() -> Self {
                Self(ulid::Ulid::new())
            }

            pub fn unprefixed(&self) -> String {
                self.0.to_string().to_lowercase()
            }
        }

        #[automatically_derived]
        impl entity_id::EntityId for #name {}

        #[automatically_derived]
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}_{}", #prefix, self.0.to_string().to_lowercase())
            }
        }

        #uuid_impls

        #[automatically_derived]
        impl std::str::FromStr for #name {
            type Err = ulid::DecodeError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let value = entity_id::unprefix_id(&s);

                Ok(Self(ulid::Ulid::from_string(value)?))
            }
        }

        #[automatically_derived]
        impl TryFrom<String> for #name {
            type Error = ulid::DecodeError;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                let value = entity_id::unprefix_id(&value);

                Ok(Self(ulid::Ulid::from_string(value)?))
            }
        }

        #[automatically_derived]
        impl TryFrom<&str> for #name {
            type Error = ulid::DecodeError;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                let value = entity_id::unprefix_id(value);

                Ok(Self(ulid::Ulid::from_string(value)?))
            }
        }
    };

    Ok(expanded.into())
}
