#[ferrocene::prevalidated]
pub fn uninstantiated_generic<T: Clone + Default>(x: T) {
    x.clone();
    let fn_type = T::default; // ok -- not actually called, and can't be resolved by HIR pass
    let mut y = fn_type();

    let fn_ptr: fn(&mut T, &T) = T::clone_from;
    fn_ptr(&mut y, &x); // ok
}
