fn main() {
    let _ = match 0 {
      0 => {
        0
      } + 5 //~ ERROR expected pattern, found `+`
    };
}

// ferrocene-annotations: fls_e5td0fa92fay
// Match Expressions
