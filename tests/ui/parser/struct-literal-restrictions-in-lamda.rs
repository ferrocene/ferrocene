struct Foo {
    x: isize,
}

impl Foo {
    fn hi(&self) -> bool {
        true
    }
}

fn main() {
    while || Foo { //~ ERROR struct literals are not allowed here
        x: 3       //~^ ERROR mismatched types
    }.hi() {
        println!("yo");
    }
}

// ferrocene-annotations: fls_5jjm1kt43axd
// While Loops
