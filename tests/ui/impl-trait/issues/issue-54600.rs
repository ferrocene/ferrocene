use std::fmt::Debug;

fn main() {
    let x: Option<impl Debug> = Some(44_u32);
    //~^ `impl Trait` only allowed in function and inherent method argument and return types
    println!("{:?}", x);
}
