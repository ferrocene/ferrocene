// run-pass
enum Wrapper {
    Wrap(i32),
}

use Wrapper::Wrap;

pub fn main() {
    let Wrap(x) = &Wrap(3);
    println!("{}", *x);

    let Wrap(x) = &mut Wrap(3);
    println!("{}", *x);

    if let Some(x) = &Some(3) {
        println!("{}", *x);
    } else {
        panic!();
    }

    if let Some(x) = &mut Some(3) {
        println!("{}", *x);
    } else {
        panic!();
    }

    if let Some(x) = &mut Some(3) {
        *x += 1;
    } else {
        panic!();
    }

    while let Some(x) = &Some(3) {
        println!("{}", *x);
        break;
    }
    while let Some(x) = &mut Some(3) {
        println!("{}", *x);
        break;
    }
    while let Some(x) = &mut Some(3) {
        *x += 1;
        break;
    }
}

// ferrocene-annotations: fls_szibmtfv117b
// Enum Types
//
// ferrocene-annotations: fls_nlzksiu8y3z9
// If and If Let Expressions
//
// ferrocene-annotations: fls_p0t1ch115tra
// If Let Expressions
//
// ferrocene-annotations: fls_yivm43r5wnp1
// Let Statements
//
// ferrocene-annotations: fls_m6kd5i9dy8dx
// While Let Loops
