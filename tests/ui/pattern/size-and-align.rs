//@ run-pass

#![allow(non_camel_case_types)]
enum clam<T> { a(T, isize), b, }

fn uhoh<T>(v: Vec<clam<T>> ) {
    match v[1] {
      clam::a::<T>(ref _t, ref u) => {
          println!("incorrect");
          println!("{}", u);
          panic!();
      }
      clam::b::<T> => { println!("correct"); }
    }
}

pub fn main() {
    let v: Vec<clam<isize>> = vec![clam::b::<isize>, clam::b::<isize>, clam::a::<isize>(42, 17)];
    uhoh::<isize>(v);
}

// ferrocene-annotations: fls_d44aflefat88
// Path Pattern Matching
//
// ferrocene-annotations: fls_uloyjbaso8pz
// Path Patterns
//
// ferrocene-annotations: fls_eexupzdsu7f
// Tuple Struct Pattern Matching
//
// ferrocene-annotations: fls_vlrto778v49m
// Tuple Struct Patterns
