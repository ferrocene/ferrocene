//@ edition:2018

type A0 = dyn;
type A1 = dyn::dyn; //~ERROR expected identifier, found keyword `dyn`
type A2 = dyn<dyn, dyn>; //~ERROR expected identifier, found `<`
type A3 = dyn<<dyn as dyn>::dyn>;

fn main() {}

// ferrocene-annotations: fls_lish33a1naw5
// Keywords
//
// ferrocene-annotations: fls_mec5cg5aptf8
// Strict Keywords
//
// ferrocene-annotations: fls_kgvleup5mdhq
// Type Aliases
