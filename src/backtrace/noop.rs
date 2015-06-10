use Frame;

#[inline(always)]
pub fn trace(_cb: &mut FnMut(&Frame) -> bool) {}
