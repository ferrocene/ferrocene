// Test slicing expressions doesn't defeat the borrow checker.

fn main() {
    let y;
    {
        let x: &[isize] = &vec![1, 2, 3, 4, 5];
        //~^ ERROR temporary value dropped while borrowed
        y = &x[1..];
    }
    y.use_ref();
}

trait Fake { fn use_mut(&mut self) { } fn use_ref(&self) { }  }
impl<T> Fake for T { }

// ferrocene-annotations: fls_vpbikb73dw4k
// Slice Types
//
// ferrocene-annotations: fls_cleoffpn5ew6
// Temporaries
//
// ferrocene-annotations: fls_a14slch83hzn
// Borrowing
//
// ferrocene-annotations: fls_qztk0bkju9u
// Borrow Expression
//
// ferrocene-annotations: fls_svkx6szhr472
// Ownership
//
// ferrocene-annotations: fls_eiw4by8z75di
// Send and Sync
//
// ferrocene-annotations: fls_izdv9i4spokw
// Operator Expressions
//
// ferrocene-annotations: fls_gho955gmob73
// Variables
