<<<PULL-UPSTREAM>>> file deleted upstream; move the Ferrocene annotations if any, and delete this file
//@ check-pass

pub fn keys<'a>(x: &'a Result<u32, u32>) -> impl std::fmt::Debug + 'a {
    match x {
        Ok(map) => Ok(map),
        Err(map) => Err(map),
    }
}

fn main() {}

// ferrocene-annotations: fls_yqcygq3y6m5j
// Lifetime
//
// ferrocene-annotations: fls_142vncdktbin
// Reference Type
