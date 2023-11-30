fn main() {
    let _ = (((),),).1.0; //~ ERROR no field `1` on type `(((),),)`

    let _ = (((),),).0.1; //~ ERROR no field `1` on type `((),)`

    let _ = (((),),).000.000; //~ ERROR no field `000` on type `(((),),)`
}

// ferrocene-annotations: fls_k64tfywtn0g8
// Tuple Expressions
//
// ferrocene-annotations: fls_18k3uajrgq5f
// Field Access Expressions
