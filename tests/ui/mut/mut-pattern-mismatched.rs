//@ dont-require-annotations: NOTE

fn main() {
    let foo = &mut 1;

    // (separate lines to ensure the spans are accurate)

     let &_ //~  ERROR mismatched types
            //~| NOTE expected mutable reference `&mut {integer}`
            //~| NOTE found reference `&_`
            //~| NOTE types differ in mutability
        = foo;
    let &mut _ = foo;

    let bar = &1;
    let &_ = bar;
    let &mut _ //~  ERROR mismatched types
               //~| NOTE expected reference `&{integer}`
               //~| NOTE found mutable reference `&mut _`
               //~| NOTE types differ in mutability
         = bar;
}

// ferrocene-annotations: fls_v5x85lt5ulva
// References
//
// ferrocene-annotations: fls_a14slch83hzn
// Borrowing
//
// ferrocene-annotations: fls_7bxv8lybxm18
// Identifier Patterns
//
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
