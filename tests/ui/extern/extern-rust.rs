//@ run-pass

#[repr(C)]
pub struct Foo(u32);

// ICE trigger, bad handling of differing types between rust and external ABIs
pub extern "C" fn bar() -> Foo {
    Foo(0)
}

fn main() {}

// ferrocene-annotations: fls_qcb1n9c0e5hz
// Functions
// ferrocene-annotations: fls_usgd0xlijoxv
// ABI
