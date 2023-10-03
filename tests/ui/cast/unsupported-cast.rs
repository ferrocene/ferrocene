struct A;

fn main() {
  println!("{:?}", 1.0 as *const A); //~ERROR  casting `f64` as `*const A` is invalid
}

// ferrocene-annotations: fls_ppd1xwve3tr7
// Raw Pointer Type
