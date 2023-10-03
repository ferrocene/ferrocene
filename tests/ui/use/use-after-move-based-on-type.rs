fn main() {
    let x = "Hello!".to_string();
    let _y = x;
    println!("{}", x); //~ ERROR borrow of moved value
}

// ferrocene-annotations: fls_svkx6szhr472
// Ownership
//
// ferrocene-annotations: fls_77scxuomlbgs
// Passing Conventions
