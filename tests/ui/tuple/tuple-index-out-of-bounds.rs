struct Point(i32, i32);

fn main() {
    let origin = Point(0, 0);
    origin.0;
    origin.1;
    origin.2;
    //~^ ERROR no field `2` on type `Point`
    let tuple = (0, 0);
    tuple.0;
    tuple.1;
    tuple.2;
    //~^ ERROR no field `2` on type `({integer}, {integer})`
}

// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
// ferrocene-annotations: fls_k64tfywtn0g8
// Tuple Expressions
//
// ferrocene-annotations: fls_18k3uajrgq5f
// Field Access Expressions
