type A = for<'a 'b> fn(); //~ ERROR expected one of `,`, `:`, or `>`, found `'b`

fn main() {}

// ferrocene-annotations: fls_yqcygq3y6m5j
// Lifetimes
