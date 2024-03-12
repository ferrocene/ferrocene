//@ check-pass

trait TraitA {
    const VALUE: usize;
}

struct A;
impl TraitA for A {
    const VALUE: usize = 1;
}

trait TraitB {
    type MyA: TraitA;
    const VALUE: usize = Self::MyA::VALUE;
}

struct B;
impl TraitB for B {
    type MyA = A;
}

fn main() {
    let _ = [0; A::VALUE];
    let _ = [0; B::VALUE]; // Indirectly refers to `A::VALUE`
}

// ferrocene-annotations: fls_l21tjqjkkaa0
// Associated items

// ferrocene-annotations: fls_e1pgdlv81vul
// Implementation conformance

// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations

// ferrocene-annotations: fls_85vx1qfa061i
// Traits
