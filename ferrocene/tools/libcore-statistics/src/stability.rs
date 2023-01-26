use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Lit, MetaNameValue};

#[derive(Clone)]
pub(crate) struct Stability {
    pub(crate) stable: bool,
    pub(crate) feature: String,
}

pub(crate) fn parse_stability(attrs: &[String]) -> Option<Stability> {
    for attr in attrs {
        let parsed_outer = syn::Attribute::parse_outer.parse_str(attr);
        let parsed_inner = syn::Attribute::parse_inner.parse_str(attr);
        let iter = parsed_outer
            .ok()
            .into_iter()
            .chain(parsed_inner.ok().into_iter())
            .flatten();

        for parsed in iter {
            let stable = if parsed.path.is_ident("stable") {
                true
            } else if parsed.path.is_ident("unstable") {
                false
            } else {
                continue;
            };

            let Ok(meta): Result<Punctuated<MetaNameValue, Comma>, _> =
                parsed.parse_args_with(Punctuated::parse_terminated) else { continue };

            for key_value in meta.iter() {
                if !key_value.path.is_ident("feature") {
                    continue;
                }
                if let Lit::Str(s) = &key_value.lit {
                    return Some(Stability {
                        stable,
                        feature: s.value(),
                    });
                }
            }
        }
    }
    None
}
