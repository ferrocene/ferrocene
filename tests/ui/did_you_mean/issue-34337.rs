fn get(key: &mut String) { }

fn main() {
    let mut v: Vec<String> = Vec::new();
    let ref mut key = v[0];
    get(&mut key); //~ ERROR cannot borrow
    //~| HELP try removing `&mut` here
}

// ferrocene-annotations: fls_qztk0bkju9u
// Borrow Expression
//
// ferrocene-annotations: fls_a14slch83hzn
// Borrowing
