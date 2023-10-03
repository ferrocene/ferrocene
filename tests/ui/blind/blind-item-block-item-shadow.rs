mod foo { pub struct Bar; }

fn main() {
    {
        struct Bar;
        use foo::Bar;
        //~^ ERROR the name `Bar` is defined multiple times
    }
}

// ferrocene-annotations: fls_9gprp17h6t1q
// Use Imports
//
// ferrocene-annotations: fls_ydmnb7qnmzzq
// Shadowing
