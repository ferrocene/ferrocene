//@ check-pass
fn main() {
    let _: i32 = (match "" {
        "+" => ::std::ops::Add::add,
        "-" => ::std::ops::Sub::sub,
        "<" => |a,b| (a < b) as i32,
        _ => unimplemented!(),
    })(5, 5);
}

// ferrocene-annotations: fls_dw33yt5g6m0k
// Type Coercion
//
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
