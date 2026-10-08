//@ known-bug: #124189
//@ edition: 2015
trait Trait {
    type Type;
}

impl<T> Trait for T {
    type Type = ();
}

fn f(_: <&Copy as Trait>::Type) {}

fn main() {
    f(());
}
