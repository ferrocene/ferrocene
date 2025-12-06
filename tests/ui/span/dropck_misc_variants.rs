//@ edition:2015..2021
// check that dropck does the right thing with misc. Ty variants

use std::fmt;
struct NoisyDrop<T: fmt::Debug>(T);
impl<T: fmt::Debug> Drop for NoisyDrop<T> {
    fn drop(&mut self) {
        let _ = vec!["0wned"];
        println!("dropping {:?}", self.0)
    }
}

trait Associator {
    type As;
}
impl<T: fmt::Debug> Associator for T {
    type As = NoisyDrop<T>;
}
struct Wrap<A: Associator>(<A as Associator>::As);

fn projection() {
    let (_w, bomb);
    bomb = vec![""];
    _w = Wrap::<&[&str]>(NoisyDrop(&bomb));
}
//~^^ ERROR `bomb` does not live long enough

fn closure() {
    let (_w,v);
    v = vec![""];
    _w = {
        let u = NoisyDrop(&v);
        //~^ ERROR `v` does not live long enough
        move || u.0.len()
    };
}

fn main() { closure(); projection() }

// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_4jiw35pan7vn
// Destruction
//
// ferrocene-annotations: fls_u2mzjgiwbkz0
// Destructors
//
// ferrocene-annotations: fls_rm4ncoopcdvj
// Drop Scopes
//
// ferrocene-annotations: fls_afafmafz4hf2
// Drop Order
//
// ferrocene-annotations: fls_85vx1qfa061i
// Traits
