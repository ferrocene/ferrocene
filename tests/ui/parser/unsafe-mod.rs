unsafe mod m {
    //~^ ERROR module cannot be declared unsafe
}

unsafe mod n;
//~^ ERROR module cannot be declared unsafe
//~^^ ERROR file not found for module `n`

fn main() {}

// ferrocene-annotations: fls_e9hwvqsib5d5
// Modules
