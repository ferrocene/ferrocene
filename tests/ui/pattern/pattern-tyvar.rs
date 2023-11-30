enum Bar { T1((), Option<Vec<isize>>), T2 }

fn foo(t: Bar) {
    match t {
      Bar::T1(_, Some::<isize>(x)) => { //~ ERROR mismatched types
        println!("{}", x);
      }
      _ => { panic!(); }
    }
}

fn main() { }

// ferrocene-annotations: fls_rce8bb7nz2jy
// Tuple Pattern Matching
//
// ferrocene-annotations: fls_urbr5rg9206v
// Tuple Patterns
//
// ferrocene-annotations: fls_yc4xm4hrfyw7
// Underscore Pattern Matching
//
// ferrocene-annotations: fls_qfsfnql1t7m
// Underscore Patterns
