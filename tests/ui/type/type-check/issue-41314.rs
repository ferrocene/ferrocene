enum X {
    Y(u32)
}

fn main() {
    match X::Y(0) {
        X::Y { number } => {}
        //~^ ERROR tuple variant `X::Y` written as struct variant
    }
}

// ferrocene-annotations: fls_nruvg0es3kx7
// Record Struct Patterns
