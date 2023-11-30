// Regression test of #86481.
struct Wrapper(i32);
struct DoubleWrapper(i32, i32);

fn main() {
    let _ = Some(3, 2); //~ ERROR this enum variant takes
    let _ = Ok(3, 6, 2); //~ ERROR this enum variant takes
    let _ = Ok(); //~ ERROR this enum variant takes
    let _ = Wrapper(); //~ ERROR this struct takes
    let _ = Wrapper(5, 2); //~ ERROR this struct takes
    let _ = DoubleWrapper(); //~ ERROR this struct takes
    let _ = DoubleWrapper(5); //~ ERROR this struct takes
    let _ = DoubleWrapper(5, 2, 7); //~ ERROR this struct takes
}

// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_8tsynkj2cufj
// Struct Expressions
