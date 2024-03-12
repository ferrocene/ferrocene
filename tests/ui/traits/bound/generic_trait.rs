//@ run-pass
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

trait connection {
    fn read(&self) -> isize;
}

trait connection_factory<C:connection> {
    fn create(&self) -> C;
}

type my_connection = ();
type my_connection_factory = ();

impl connection for () {
    fn read(&self) -> isize { 43 }
}

impl connection_factory<my_connection> for my_connection_factory {
    fn create(&self) -> my_connection { () }
}

pub fn main() {
    let factory = ();
    let connection: () = factory.create();
    let result = connection.read();
    assert_eq!(result, 43);
}

// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
//
// ferrocene-annotations: fls_jeoas4n6su4
// Trait and Lifetime Bounds
//
// ferrocene-annotations: fls_ikfvbeewame7
// Subtyping and Variance
//
// ferrocene-annotations: fls_qa98qdi42orq
// Trait Object Types
//
// ferrocene-annotations: fls_vhpwge5123cm
// Generic Parameters
