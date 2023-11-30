// Regression test for #93927: suggested trait bound for T should be Eq, not PartialEq
struct MyType<T>(T);

impl<T> PartialEq for MyType<T>
where
    T: Eq,
{
    fn eq(&self, other: &Self) -> bool {
        true
    }
}

fn cond<T: PartialEq>(val: MyType<T>) -> bool {
    val == val
    //~^ ERROR binary operation `==` cannot be applied to type `MyType<T>`
}

fn main() {
    cond(MyType(0));
}

// ferrocene-annotations: fls_nsvzzbldhq53
// Comparison Expressions
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
