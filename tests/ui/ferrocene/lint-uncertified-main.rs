#![crate_type = "bin"]
#![deny(ferrocene::uncertified)]

fn normal_def() {}

fn main() {
    normal_def();
    //~^ ERROR unvalidated
}
