//@ run-pass

pub fn main() {
    let x: isize = 3;
    println!("&x={:x}", (&x as *const isize as usize));
}

// ferrocene-annotations: fls_ppd1xwve3tr7
// Raw Pointer Type
