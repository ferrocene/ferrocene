//@ edition: 2015
fn dyn() -> &'static dyn use<> { &() }
//~^ ERROR expected one of `!`, `(`, `::`, `<`, `where`, or `{`, found keyword `use`
