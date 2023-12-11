fn main() {
  match 42 {
    x < 7 => (),
   //~^ error: expected one of `,`, `=>`, `@`, `if`, `|`, or `}`, found `<`
    _ => ()
  }
}

// ferrocene-annotations: fls_xgqh0ju6bmbn
// Patterns
