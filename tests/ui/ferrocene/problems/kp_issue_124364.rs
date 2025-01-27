//@ run-pass
//@ compile-flags: -O

#[inline(never)]
fn throw_if_over_300(i: usize) {
    println!("{i}");
    if i > 300 { unreachable!("we should not end up here, we only call this for <= 300") }
}

#[inline(never)]
pub fn evil() {
    const INC: f64 = f64::MAX / 90.0;
    let mut x: f64 = -1.0;
    let mut i: usize = 0;

    while x.is_sign_negative() {
        if i <= 300 {
            throw_if_over_300(i);
        } else {
            break;
        }
        // Use a big decrement to reach negative infinity quickly.
        x -= INC;
        // Convert infinity into a NaN (Inf - Inf = NaN)
        x = x + x - x;
        i += 2;
    }
}

pub fn main() {
    evil();
}
