struct Foo<'a,'b> {
    x: &'a isize,
    y: &'b isize,
}

impl<'a,'b> Foo<'a,'b> {
    fn bar(self:
           Foo<'b,'a>
    //~^ ERROR mismatched `self` parameter type
    //~| expected struct `Foo<'a, 'b>`
    //~| found struct `Foo<'b, 'a>`
    //~| lifetime mismatch
    //~| ERROR mismatched `self` parameter type
    //~| expected struct `Foo<'a, 'b>`
    //~| found struct `Foo<'b, 'a>`
    //~| lifetime mismatch
           ) {}
}

fn main() {}

// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
// ferrocene-annotations: fls_yqcygq3y6m5j
// Lifetimes
