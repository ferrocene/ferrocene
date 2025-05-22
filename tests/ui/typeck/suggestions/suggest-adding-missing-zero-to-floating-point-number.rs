//@ run-rustfix

fn main() {
    2.e1; //~ERROR `{integer}` is a primitive type and therefore doesn't have fields
    2.E1; //~ERROR `{integer}` is a primitive type and therefore doesn't have fields
    2.f32; //~ERROR `{integer}` is a primitive type and therefore doesn't have fields
    2.f64; //~ERROR `{integer}` is a primitive type and therefore doesn't have fields
    2.e+12; //~ERROR `{integer}` is a primitive type and therefore doesn't have fields
    2.e-12; //~ERROR `{integer}` is a primitive type and therefore doesn't have fields
    2.e1f32; //~ERROR `{integer}` is a primitive type and therefore doesn't have fields
}

// ferrocene-annotations: fls_18k3uajrgq5f
// Field Access Expressions
//
// ferrocene-annotations: fls_6ydylimiv553
// Place Expressions
//
// ferrocene-annotations: fls_xcwfotmq2e5d
// Field Resolution
