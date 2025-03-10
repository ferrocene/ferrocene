//@ run-pass
//@ compile-flags: -O

fn main() {
    assert!(core::any::TypeId::of::<u8>() != core::any::TypeId::of::<(u8, u8)>());
    assert!(!can_cast_u8_to_tuple_u8_u8());
}

pub fn can_cast_u8_to_tuple_u8_u8() -> bool {
    fn cast_u8_to_tuple_u8_u8(value: u8) -> Result<(u8, u8), u8> {
        if core::any::TypeId::of::<u8>() == core::any::TypeId::of::<(u8, u8)>() {
            Ok(unsafe { std::ptr::read(&value as *const u8 as *const (u8, u8)) })
        } else {
            Err(value)
        }
    }

    cast_u8_to_tuple_u8_u8(123_u8).is_ok()
}
