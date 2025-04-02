<<<PULL-UPSTREAM>>> file deleted upstream; move the Ferrocene annotations if any, and delete this file
struct Foo {
    x: isize,
}

impl Foo {
    fn hi(&self) -> bool {
        true
    }
}

fn main() {
    while Foo { //~ ERROR struct literals are not allowed here
        x: 3
    }.hi() {
        println!("yo");
    }
    while let true = Foo { //~ ERROR struct literals are not allowed here
        x: 3
    }.hi() {
        println!("yo");
    }
}

// ferrocene-annotations: fls_5jjm1kt43axd
// While Loops
