//@ check-pass

use std::fmt::Debug;

#[derive(Debug)]
pub struct Target;

#[derive(Debug)]
pub struct Source;
impl From<Source> for Target {
    fn from(_: Source) -> Self {
        Self
    }
}

fn maybe_source() -> Result<(), Source> {
    todo!()
}

pub fn typaram() -> Result<(), impl Debug> {
    maybe_source()?;
    Ok::<_, Target>(())
}

pub fn direct() -> Result<(), impl Debug> {
    maybe_source()?;
    Err(Target)
}

fn main() {}

// ferrocene-annotations: fls_pocsh1neugpc
// Error Propagation Expression
//
// ferrocene-annotations: fls_izdv9i4spokw
// Operator Expressions
