trait Tr {
    type TrSubtype;
}

struct Bar<'a, Item: Tr, <Item as Tr>::TrSubtype: 'a> {
    //~^ ERROR bounds on associated types do not belong here
    item: Item,
    item_sub: &'a <Item as Tr>::TrSubtype,
}

fn main() {}

// ferrocene-annotations: fls_yqcygq3y6m5j
// Lifetimes
//
// ferrocene-annotations: fls_jeoas4n6su4
// Trait and Lifetime Bounds
