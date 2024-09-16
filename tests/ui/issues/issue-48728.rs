// Regression test for #48728, an ICE that occurred computing
// coherence "help" information.

//@ revisions: current next
//@ ignore-compare-mode-next-solver (explicit revisions)
//@[next] compile-flags: -Znext-solver
//@[next] check-pass

#[derive(Clone)] //[current]~ ERROR conflicting implementations of trait `Clone`
struct Node<T: ?Sized>(Box<T>);

impl<T: Clone + ?Sized> Clone for Node<[T]> {
    fn clone(&self) -> Self {
        Node(Box::clone(&self.0))
    }
}

fn main() {}
