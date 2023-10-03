#![feature(never_type)]

fn cast_a() {
    let y = {return; 22} as !;
    //~^ ERROR non-primitive cast
}

fn cast_b() {
    let y = 22 as !; //~ ERROR non-primitive cast
}

fn main() { }

// ferrocene-annotations: fls_98lnexk53ru4
// Never Type
