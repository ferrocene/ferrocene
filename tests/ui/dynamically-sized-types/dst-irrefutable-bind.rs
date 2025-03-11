//@ run-pass

struct Test<T: ?Sized>(T);

fn main() {
    let x = Test([1,2,3]);
    let x : &Test<[i32]> = &x;

    let & ref _y = x;

    // Make sure binding to a fat pointer behind a reference
    // still works
    let slice = &[1,2,3];
    let x = Test(&slice);
    let Test(&_slice) = x;
}

// ferrocene-annotations: fls_qztk0bkju9u
// Borrow Expression
// ferrocene-annotations: fls_7bxv8lybxm18
// Identifier Patterns
//
// ferrocene-annotations: fls_izdv9i4spokw
// Operator Expressions
