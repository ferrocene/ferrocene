//@ compile-flags: -Zdeduplicate-diagnostics=yes

// This test of structural match checking enumerates the different kinds of
// const definitions, collecting cases where the const pattern is rejected.
//
// Note: Even if a non-structural-match type is part of an expression in a
// const's definition, that does not necessarily disqualify the const from being
// a match pattern: in principle, we just need the types involved in the final
// value to be structurally matchable.

// See also RFC 1445

#![feature(type_ascription)]

#[derive(Copy, Clone, Debug)]
struct NoPartialEq;

#[derive(Copy, Clone, Debug)]
struct NoDerive;

// This impl makes `NoDerive` irreflexive.
impl PartialEq for NoDerive { fn eq(&self, _: &Self) -> bool { false } }

impl Eq for NoDerive { }

type OND = Option<NoDerive>;

struct TrivialEq(OND);

// This impl makes `TrivialEq` trivial.
impl PartialEq for TrivialEq { fn eq(&self, _: &Self) -> bool { true } }

impl Eq for TrivialEq { }

fn main() {
    #[derive(PartialEq, Eq, Debug)]
    enum Derive<X> { Some(X), None, }

    const ENUM: Derive<NoDerive> = Derive::Some(NoDerive);
    match Derive::Some(NoDerive) { ENUM => dbg!(ENUM), _ => panic!("whoops"), };
    //~^ ERROR must be annotated with `#[derive(PartialEq)]`
    //~| NOTE the traits must be derived
    //~| NOTE StructuralPartialEq.html for details

    const FIELD: OND = TrivialEq(Some(NoDerive)).0;
    match Some(NoDerive) { FIELD => dbg!(FIELD), _ => panic!("whoops"), };
    //~^ ERROR must be annotated with `#[derive(PartialEq)]`
    //~| NOTE the traits must be derived
    //~| NOTE StructuralPartialEq.html for details

    const NO_DERIVE_SOME: OND = Some(NoDerive);
    const INDIRECT: OND = NO_DERIVE_SOME;
    match Some(NoDerive) {INDIRECT => dbg!(INDIRECT), _ => panic!("whoops"), };
    //~^ ERROR must be annotated with `#[derive(PartialEq)]`
    //~| NOTE the traits must be derived
    //~| NOTE StructuralPartialEq.html for details

    const TUPLE: (OND, OND) = (None, Some(NoDerive));
    match (None, Some(NoDerive)) { TUPLE => dbg!(TUPLE), _ => panic!("whoops"), };
    //~^ ERROR must be annotated with `#[derive(PartialEq)]`
    //~| NOTE the traits must be derived
    //~| NOTE StructuralPartialEq.html for details

    const TYPE_ASCRIPTION: OND = type_ascribe!(Some(NoDerive), OND);
    match Some(NoDerive) { TYPE_ASCRIPTION => dbg!(TYPE_ASCRIPTION), _ => panic!("whoops"), };
    //~^ ERROR must be annotated with `#[derive(PartialEq)]`
    //~| NOTE the traits must be derived
    //~| NOTE StructuralPartialEq.html for details

    const ARRAY: [OND; 2] = [None, Some(NoDerive)];
    match [None, Some(NoDerive)] { ARRAY => dbg!(ARRAY), _ => panic!("whoops"), };
    //~^ ERROR must be annotated with `#[derive(PartialEq)]`
    //~| NOTE the traits must be derived
    //~| NOTE StructuralPartialEq.html for details

    const REPEAT: [OND; 2] = [Some(NoDerive); 2];
    match [Some(NoDerive); 2] { REPEAT => dbg!(REPEAT), _ => panic!("whoops"), };
    //~^ ERROR must be annotated with `#[derive(PartialEq)]`
    //~| NOTE the traits must be derived
    //~| NOTE StructuralPartialEq.html for details

    trait Trait: Sized { const ASSOC: Option<Self>; }
    impl Trait for NoDerive { const ASSOC: Option<NoDerive> = Some(NoDerive); }
    match Some(NoDerive) { NoDerive::ASSOC => dbg!(NoDerive::ASSOC), _ => panic!("whoops"), };
    //~^ ERROR must be annotated with `#[derive(PartialEq)]`
    //~| NOTE the traits must be derived
    //~| NOTE StructuralPartialEq.html for details

    const BLOCK: OND = { NoDerive; Some(NoDerive) };
    match Some(NoDerive) { BLOCK => dbg!(BLOCK), _ => panic!("whoops"), };
    //~^ ERROR must be annotated with `#[derive(PartialEq)]`
    //~| NOTE the traits must be derived
    //~| NOTE StructuralPartialEq.html for details

    const ADDR_OF: &OND = &Some(NoDerive);
    match &Some(NoDerive) { ADDR_OF => dbg!(ADDR_OF), _ => panic!("whoops"), };
    //~^ ERROR must be annotated with `#[derive(PartialEq)]`
    //~| NOTE the traits must be derived
    //~| NOTE StructuralPartialEq.html for details
}
