//@ edition: 2015
fn main() {
    panic!(std::default::Default::default());
    //~^ ERROR type annotations needed
}
