//! Functions and types dealing with attributes and meta items.
//!
//! FIXME(Centril): For now being, much of the logic is still in `rustc_ast::attr`.
//! The goal is to move the definition of `MetaItem` and things that don't need to be in `syntax`
//! to this crate.

// tidy-alphabetical-start
#![allow(internal_features)]
#![doc(rust_logo)]
#![feature(let_chains)]
#![feature(rustdoc_internals)]
// tidy-alphabetical-end

mod builtin;
mod session_diagnostics;

pub use builtin::*;
pub use rustc_ast::attr::*;
pub(crate) use rustc_session::HashStableContext;
pub use IntType::*;
pub use ReprAttr::*;
pub use StabilityLevel::*;

rustc_fluent_macro::fluent_messages! { "../messages.ftl" }
