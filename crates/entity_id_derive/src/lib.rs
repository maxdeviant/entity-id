mod internals;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{self, parse_macro_input, DeriveInput, Token};

use internals::symbols::*;

#[proc_macro_derive(EntityId, attributes(entity_id))]
pub fn entity_id(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);

    expand_derive_entity_id(&mut input)
        .unwrap_or_else(to_compile_errors)
        .into()
}

fn to_compile_errors(errors: Vec<syn::Error>) -> proc_macro2::TokenStream {
    let compile_errors = errors.iter().map(syn::Error::to_compile_error);
    quote!(#(#compile_errors)*)
}

fn expand_derive_entity_id(
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

    Ok(expanded.into())
}
