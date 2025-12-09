use core::convert::AsMut;

#[test]
fn convert() {
    const fn from(x: i32) -> i32 {
        i32::from(x)
    }

    const FOO: i32 = from(42);
    assert_eq!(FOO, 42);

    const fn into(x: Vec<String>) -> Vec<String> {
        x.into()
    }

    const BAR: Vec<String> = into(Vec::new());
    assert_eq!(BAR, Vec::<String>::new());

    assert_eq!(FOO, core::convert::identity(FOO));
}

#[test]
fn as_mut_impls() {
    let mut s = String::from("hello");
    let mut s_mut = &mut s;
    let slice: &mut str = <&mut String as AsMut<str>>::as_mut(&mut s_mut);
    let _: &mut str = <str as AsMut<str>>::as_mut(slice);

    let mut b = Vec::from(b"hello");
    let _: &mut [u8] = <[u8] as AsMut<[u8]>>::as_mut(b.as_mut_slice());
}

#[test]
fn into_as_try_into() {
    struct A;
    struct B;

    impl Into<B> for A {
        fn into(self) -> B {
            B
        }
    }

    // This wouldn't compile if the `TryInto`/`TryFrom` blanket impls used
    // `U: From<T>` instead of `T: Into<U>`
    let Ok(B) = A.try_into();
    let Ok(B) = B::try_from(A);
}
