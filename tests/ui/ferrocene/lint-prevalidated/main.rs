#![deny(ferrocene::unvalidated)]

fn normal_def() {}

fn main() {
    normal_def();
    //~^ ERROR unvalidated
}
