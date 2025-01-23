// `ty` matcher accepts trait object types

macro_rules! m {
    ($t: ty) => ( let _: $t; )
}

fn main() {
    m!(dyn Copy + Send + 'static);
    //~^ ERROR the trait `Copy` is not dyn compatible
    m!(dyn 'static + Send);
    m!(dyn 'static +); //~ ERROR at least one trait is required for an object type
}

// ferrocene-annotations: fls_8nzypdu9j3ge
// Metavariables
// ferrocene-annotations: fls_vnvt40pa48n8
// Macro Invocation
