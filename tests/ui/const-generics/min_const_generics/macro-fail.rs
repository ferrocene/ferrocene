struct Example<const N: usize>;

macro_rules! external_macro {
  () => {{
    //~^ ERROR expected type
    const X: usize = 1337;
    X
  }}
}

trait Marker<const N: usize> {}
impl<const N: usize> Marker<N> for Example<N> {}

fn make_marker() -> impl Marker<gimme_a_const!(marker)> {
  //~^ ERROR: type provided when a constant was expected
  //~| ERROR: type provided when a constant was expected
  Example::<gimme_a_const!(marker)>
  //~^ ERROR: type provided when a constant was expected
}

fn from_marker(_: impl Marker<{
    #[macro_export]
    macro_rules! inline { () => {{ 3 }} }; inline!()
}>) {}

fn main() {
  let _ok = Example::<{
    #[macro_export]
    macro_rules! gimme_a_const {
      ($rusty: ident) => {{ let $rusty = 3; *&$rusty }}
      //~^ ERROR expected type
      //~| ERROR expected type
    };
    gimme_a_const!(run)
  }>;

  let _fail = Example::<external_macro!()>;
  //~^ ERROR: type provided when a constant was expected

  let _fail = Example::<gimme_a_const!()>;
  //~^ ERROR unexpected end of macro invocation
  //~| ERROR: type provided when a constant was expected
}

// ferrocene-annotations: fls_xa7lp0zg1ol2
// Declarative macros

// ferrocene-annotations: fls_wjldgtio5o75
// Macro expansion

// ferrocene-annotations: fls_vnvt40pa48n8
// Macro invocation

// ferrocene-annotations: fls_4apk1exafxii
// Macro matching

// ferrocene-annotations: fls_ym00b6ewf4n3
// Macro transcription

// ferrocene-annotations: fls_n3ktmjqf87qb
// Rule matching

// ferrocene-annotations: fls_qpx6lgapce57
// Token matching
