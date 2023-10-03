macro_rules! num { () => { 1 } }

fn main() {
    let x = 1i32;
    x.e10; //~ERROR `i32` is a primitive type and therefore doesn't have fields

    let y = 1;
    y.e10; //~ERROR `{integer}` is a primitive type and therefore doesn't have fields

    2u32.e10; //~ERROR `u32` is a primitive type and therefore doesn't have fields

    num!().e10; //~ERROR `{integer}` is a primitive type and therefore doesn't have fields

    2.e10foo; //~ERROR `{integer}` is a primitive type and therefore doesn't have fields

    42._;
    //~^ERROR expected identifier, found reserved identifier `_`
    //~|ERROR `{integer}` is a primitive type and therefore doesn't have fields

    42.a; //~ERROR `{integer}` is a primitive type and therefore doesn't have fields
}

// ferrocene-annotations: fls_18k3uajrgq5f
// Field Access Expressions
//
// ferrocene-annotations: fls_xcwfotmq2e5d
// Field Resolution
