//@ run-pass
// Destructuring struct variants would ICE where regular structs wouldn't

enum Foo {
    VBar { num: isize }
}

struct SBar { num: isize }

pub fn main() {
    let vbar = Foo::VBar { num: 1 };
    let Foo::VBar { num } = vbar;
    assert_eq!(num, 1);

    let sbar = SBar { num: 2 };
    let SBar { num } = sbar;
    assert_eq!(num, 2);
}

// ferrocene-annotations: fls_vnai6ag4qrdb
// Identifier Pattern Matching
//
// ferrocene-annotations: fls_7bxv8lybxm18
// Identifier Patterns
//
// ferrocene-annotations: fls_nruvg0es3kx7
// Record Struct Patterns
//
// ferrocene-annotations: fls_asj8rgccvkoe
// Struct Pattern Matching
//
// ferrocene-annotations: fls_7dbd5t2750ce
// Struct Patterns
