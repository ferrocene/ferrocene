struct S;

fn repro_ref(thing: S) {
    thing(); //~ ERROR expected function, found `S`
}

fn main() {}

// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
//
// ferrocene-annotations: fls_qcb1n9c0e5hz
// Functions
