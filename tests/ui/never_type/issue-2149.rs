trait VecMonad<A> {
    fn bind<B, F>(&self, f: F) where F: FnMut(A) -> Vec<B>;
}

impl<A> VecMonad<A> for Vec<A> {
    fn bind<B, F>(&self, mut f: F) where F: FnMut(A) -> Vec<B> {
        let mut r = panic!();
        for elt in self { r = r + f(*elt); }
        //~^ ERROR E0277
   }
}
fn main() {
    ["hi"].bind(|x| [x] );
    //~^ ERROR no method named `bind` found
}

// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
