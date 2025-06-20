#[test]
fn test_blanket_impl_for_ref() {
    let f = |a: i32| a + 1;

    // Fn<A> for &F where F: Fn<A>
    assert_eq!((&f).call((2,)), 3);
    // FnMut<A> for &F where F: Fn<A>
    assert_eq!((&f).call_mut((3,)), 4);
    // FnOnce<A> for &F where F: Fn<A>
    assert_eq!((&f).call_once((4,)), 5);
}

#[test]
fn test_blanket_impl_for_mut_ref() {
    let mut a = 0;
    {
        let mut f_mut = |b: i32| a += b;

        // FnMut<A> for &mut F where F: FnMut<A>
        FnMut::call_mut(&mut f_mut, (1,));
        // FnOnce<A> for &mut F where F: FnMut<A>
        FnOnce::call_once(f_mut, (2,));
    }
    assert_eq!(a, 3);
}
