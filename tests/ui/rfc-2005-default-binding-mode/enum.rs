enum Wrapper {
    Wrap(i32),
}

use Wrapper::Wrap;

pub fn main() {
    let Wrap(x) = &Wrap(3);
    *x += 1; //~ ERROR cannot assign to `*x`, which is behind a `&` reference


    if let Some(x) = &Some(3) {
        *x += 1; //~ ERROR cannot assign to `*x`, which is behind a `&` reference
    } else {
        panic!();
    }

    while let Some(x) = &Some(3) {
        *x += 1; //~ ERROR cannot assign to `*x`, which is behind a `&` reference
        break;
    }
}

// ferrocene-annotations: fls_m6kd5i9dy8dx
// While Let Loops
// ferrocene-annotations: fls_p0t1ch115tra
// If Let Expressions
// ferrocene-annotations: fls_vlrto778v49m
// Tuple Struct Patterns
// ferrocene-annotations: fls_eexupzdsu7f
// Tuple Struct Pattern Matching
