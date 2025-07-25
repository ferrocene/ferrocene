macro_rules! foo()  //~ ERROR semicolon
                    //~| ERROR macros must contain at least one rule

macro_rules! bar {
    ($($tokens:tt)*) => {}
}

bar!( //~ ERROR semicolon
    blah
    blah
    blah
)

fn main() {
}

// ferrocene-annotations: fls_xa7lp0zg1ol2
// Declarative Macros
//
// ferrocene-annotations: fls_vnvt40pa48n8
// Macro Invocation
