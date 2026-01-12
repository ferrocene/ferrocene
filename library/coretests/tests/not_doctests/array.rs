// cover `<core::array::drain::Drain<'_, T> as core::iter::traits::iterator::Iterator>::next`.
#[test]
fn test_drain_next() {
    core::array::ferrocene_test::test_drain_next();
}
