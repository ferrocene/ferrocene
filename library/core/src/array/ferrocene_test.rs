use super::*;

pub fn test_drain_call_once() {
    let mut md = ManuallyDrop::new([1, 2, 3, 4, 5]);
    let mut f = |c| c + 10;
    // SAFETY: is only called once
    let mut drain = unsafe { drain::Drain::new(&mut md, &mut f) };
    assert_eq!(11, drain.call_once((15,)));
}
