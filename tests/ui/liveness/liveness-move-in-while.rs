fn main() {

    let y: Box<isize> = 42.into();
    let mut x: Box<isize>;

    loop {
        println!("{}", y); //~ ERROR borrow of moved value: `y`
        while true { while true { while true { x = y; x.clone(); } } }
        //~^ WARN denote infinite loops with
        //~| WARN denote infinite loops with
        //~| WARN denote infinite loops with
    }
}

// ferrocene-annotations: fls_a14slch83hzn
// Borrowing
//
// ferrocene-annotations: fls_77scxuomlbgs
// Passing Conventions
