// Ensure that fn main is implicitly ferrocene::prevalidated in bin crates

#![deny(ferrocene::unvalidated)]

fn normal_def() {}

fn main() {
    normal_def();
    //~^ ERROR unvalidated
}
