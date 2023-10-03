struct Point { x: isize, y: isize }
struct Empty;

fn main() {
    let origin = Point { x: 0, y: 0 };
    origin.0;
    //~^ ERROR no field `0` on type `Point`
    Empty.0;
    //~^ ERROR no field `0` on type `Empty`
}

// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_18k3uajrgq5f
// Field Access Expressions
