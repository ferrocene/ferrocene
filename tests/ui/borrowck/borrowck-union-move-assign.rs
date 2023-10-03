use std::mem::ManuallyDrop;

// Non-copy
struct A;
struct B;

union U {
    a: ManuallyDrop<A>,
    b: ManuallyDrop<B>,
}

fn main() {
    unsafe {
        {
            let mut u = U { a: ManuallyDrop::new(A) };
            let a = u.a;
            let a = u.a; //~ ERROR use of moved value: `u`
        }
        {
            let mut u = U { a: ManuallyDrop::new(A) };
            let a = u.a;
            u.a = ManuallyDrop::new(A);
            let a = u.a; // OK
        }
        {
            let mut u = U { a: ManuallyDrop::new(A) };
            let a = u.a;
            u.b = ManuallyDrop::new(B);
            let a = u.a; // OK
        }
    }
}

//
// ferrocene-annotations: fls_fmdn7n7s413d
// Union Type
//
// ferrocene-annotations: fls_9kjpxri0axvg
// Weak Keywords
