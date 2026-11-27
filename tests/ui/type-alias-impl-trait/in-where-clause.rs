// Ferrocene addition: The failing query stack in this test differs between panic=unwind and
// panic=abort configurations, leading to a different set of error messages
//
//@ revisions: unwind abort
//@ [unwind] needs-unwind
//@ [abort] compile-flags: -C panic=abort

//! We evaluate `1 + 2` with `TypingMode::PostAnalysis` during typeck, causing
//! us to get the concrete type of `Bar` while computing it.
//! This again requires type checking `foo`.
#![feature(type_alias_impl_trait)]
type Bar = impl Sized;
//~^ ERROR: cycle

#[define_opaque(Bar)]
fn foo() -> Bar
where
    Bar: Send,
{
    [0; 1 + 2]
}

fn main() {}
