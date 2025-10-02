//@ revisions: e2024 e2015
//@[e2024] edition: 2024
//@[e2015] edition: 2015

fn test_gen() {
    gen {};
    //[e2015]~^ ERROR: cannot find struct, variant or union type `gen`
    //[e2024]~^^ ERROR: gen blocks are experimental
    //[e2024]~| ERROR: type annotations needed
}

fn test_async_gen() {
    async gen {};
    //[e2015]~^ ERROR expected one of `!`, `.`, `::`, `;`, `?`, `{`, `}`, or an operator, found `gen`
    //[e2024]~^^ ERROR: gen blocks are experimental
    //[e2024]~| ERROR: type annotations needed
}

fn main() {}

#[cfg(false)]
fn foo() {
    gen {};
    //[e2024]~^ ERROR: gen blocks are experimental

    async gen {};
    //[e2024]~^ ERROR: gen blocks are experimental
    //[e2015]~^^ ERROR expected one of `!`, `.`, `::`, `;`, `?`, `{`, `}`, or an operator, found `gen`
}
