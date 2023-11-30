fn main() {
  let isize x = 5; //~ ERROR expected one of `:`, `;`, `=`, `@`, or `|`, found `x`
  match x;
}

// ferrocene-annotations: fls_yivm43r5wnp1
// Let Statements
