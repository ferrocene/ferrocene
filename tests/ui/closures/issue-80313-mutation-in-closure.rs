fn main() {
    let mut my_var = false;
    let callback = || {
        my_var = true;
    };
    callback(); //~ ERROR E0596
}

// ferrocene-annotations: fls_a14slch83hzn
// Borrowing
