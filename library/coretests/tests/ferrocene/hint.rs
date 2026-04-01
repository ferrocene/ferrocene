/// This test proves that the destructor
/// `<core::hint::select_unpredictable::DropOnPanic<T> as core::ops::drop::Drop>::drop`
/// is being executed, even if it is not shown as covered in the coverage report.
#[test]
#[should_panic = "0"]
fn test_select_unpredictable_runs_destructor() {
    struct Foo(i32);

    impl Drop for Foo {
        fn drop(&mut self) {
            if self.0 == 0 {
                panic!("0");
            }
        }
    }

    core::hint::select_unpredictable(true, Foo(0), Foo(1));

    unreachable!("the destructor will panic before");
}
