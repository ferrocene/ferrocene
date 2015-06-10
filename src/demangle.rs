// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt::{self, Write};

// All rust symbols are in theory lists of "::"-separated identifiers. Some
// assemblers, however, can't handle these characters in symbol names. To get
// around this, we use C++-style mangling. The mangling method is:
//
// 1. Prefix the symbol with "_ZN"
// 2. For each element of the path, emit the length plus the element
// 3. End the path with "E"
//
// For example, "_ZN4testE" => "test" and "_ZN3foo3bar" => "foo::bar".
//
// We're the ones printing our backtraces, so we can't rely on anything else to
// demangle our symbols. It's *much* nicer to look at demangled symbols, so
// this function is implemented to give us nice pretty output.
//
// Note that this demangler isn't quite as fancy as it could be. We have lots
// of other information in our symbols like hashes, version, type information,
// etc. Additionally, this doesn't handle glue symbols at all.
pub fn demangle(writer: &mut Write, s: &str) -> fmt::Result {
    // First validate the symbol. If it doesn't look like anything we're
    // expecting, we just print it literally. Note that we must handle non-rust
    // symbols because we could have any function in the backtrace.
    let mut valid = true;
    let mut inner = s;
    if s.len() > 4 && s.starts_with("_ZN") && s.ends_with("E") {
        inner = &s[3 .. s.len() - 1];
    // On Windows, dbghelp strips leading underscores, so we accept "ZN...E"
    // form too.
    } else if s.len() > 3 && s.starts_with("ZN") && s.ends_with("E") {
        inner = &s[2 .. s.len() - 1];
    } else {
        valid = false;
    }

    if valid {
        let mut chars = inner.chars();
        while valid {
            let mut i = 0;
            for c in chars.by_ref() {
                if c.is_numeric() {
                    i = i * 10 + c as usize - '0' as usize;
                } else {
                    break
                }
            }
            if i == 0 {
                valid = chars.next().is_none();
                break
            } else if chars.by_ref().take(i - 1).count() != i - 1 {
                valid = false;
            }
        }
    }

    // Alright, let's do this.
    if !valid {
        return writer.write_str(s);
    }

    let mut first = true;
    while !inner.is_empty() {
        if !first {
            try!(writer.write_str("::"));
        } else {
            first = false;
        }
        let mut rest = inner;
        while rest.chars().next().unwrap().is_numeric() {
            rest = &rest[1..];
        }
        let i: usize = inner[..(inner.len() - rest.len())].parse().unwrap();
        inner = &rest[i..];
        rest = &rest[..i];
        while !rest.is_empty() {
            if rest.starts_with("$") {
                macro_rules! demangle {
                    ($($pat:expr, => $demangled:expr),*) => ({
                        $(if rest.starts_with($pat) {
                            try!(writer.write_str($demangled));
                            rest = &rest[$pat.len()..];
                          } else)*
                        {
                            try!(writer.write_str(rest));
                            break;
                        }

                    })
                }

                // see src/librustc/back/link.rs for these mappings
                demangle! {
                    "$SP$", => "@",
                    "$BP$", => "*",
                    "$RF$", => "&",
                    "$LT$", => "<",
                    "$GT$", => ">",
                    "$LP$", => "(",
                    "$RP$", => ")",
                    "$C$", => ",",

                    // in theory we can demangle any Unicode code point, but
                    // for simplicity we just catch the common ones.
                    "$u7e$", => "~",
                    "$u20$", => " ",
                    "$u27$", => "'",
                    "$u5b$", => "[",
                    "$u5d$", => "]"
                }
            } else {
                let idx = rest.find('$').unwrap_or(rest.len());
                try!(writer.write_str(&rest[..idx]));
                rest = &rest[idx..];
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    macro_rules! t { ($a:expr, $b:expr) => ({
        let mut m = String::new();
        super::demangle(&mut m, $a).unwrap();
        assert_eq!(m, $b);
    }) }

    #[test]
    fn demangle() {
        t!("test", "test");
        t!("_ZN4testE", "test");
        t!("_ZN4test", "_ZN4test");
        t!("_ZN4test1a2bcE", "test::a::bc");
    }

    #[test]
    fn demangle_dollars() {
        t!("_ZN4$RP$E", ")");
        t!("_ZN8$RF$testE", "&test");
        t!("_ZN8$BP$test4foobE", "*test::foob");
        t!("_ZN9$u20$test4foobE", " test::foob");
    }

    #[test]
    fn demangle_many_dollars() {
        t!("_ZN13test$u20$test4foobE", "test test::foob");
        t!("_ZN12test$BP$test4foobE", "test*test::foob");
    }

    #[test]
    fn demangle_windows() {
        t!("ZN4testE", "test");
        t!("ZN13test$u20$test4foobE", "test test::foob");
        t!("ZN12test$RF$test4foobE", "test&test::foob");
    }
}
