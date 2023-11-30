mod a {
}

trait A {
}

impl A for a { //~ ERROR expected type, found module
}

fn main() {
}

// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
