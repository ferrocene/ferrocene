fn main() {
    let mut my_var = false;
    let callback = move || {
        &mut my_var;
    };
    callback(); //~ ERROR E0596
}

// ferrocene-annotations: fls_a14slch83hzn
// Borrowing
