union Test {
    a: A, //~ ERROR unions cannot contain fields that may need dropping
    b: B
}

#[derive(Debug)]
struct A(i32);
impl Drop for A {
    fn drop(&mut self) { println!("A"); }
}

#[derive(Debug)]
struct B(f32);
impl Drop for B {
    fn drop(&mut self) { println!("B"); }
}

fn main() {
    let mut test = Test { a: A(3) };
    println!("{:?}", unsafe { test.b });
    unsafe { test.b = B(0.5); }
}

// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_jep7p27kaqlp
// Unsafety
//
// ferrocene-annotations: fls_18k3uajrgq5f
// Field Access Expressions
//
// ferrocene-annotations: fls_xcwfotmq2e5d
// Field Resolution
