/* FIXME(#110395)
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
}
*/

#[test]
fn as_ref_for_mut_ref() {
    let mut a = "as_ref".to_string();
    let b = &mut a;
    let _c: &str = b.as_ref();
}

#[test]
fn as_mut_for_mut_ref() {
    let mut a = "as_mut".to_string();
    let b = &mut a;
    let _c: &mut str = b.as_mut();
}

#[test]
fn as_mut_for_slice() {
    let mut a = vec![1, 2, 3, 4, 5, 6].into_boxed_slice();
    let _b: &mut [i32] = (*a).as_mut();
}

#[test]
fn as_mut_for_str() {
    let mut a: Box<str> = "as_mut".to_string().into_boxed_str();
    let _b: &mut str = (*a).as_mut();
}
