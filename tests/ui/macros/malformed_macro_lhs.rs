macro_rules! my_precioooous {
    t => (1); //~ ERROR invalid macro matcher
}

fn main() {
    my_precioooous!();
}

// ferrocene-annotations: fls_xa7lp0zg1ol2
// Declarative Macros
