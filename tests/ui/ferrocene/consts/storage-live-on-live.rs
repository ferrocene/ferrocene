//@ check-pass
#![feature(core_intrinsics, custom_mir)]
use std::intrinsics::mir::*;

#[custom_mir(dialect = "built")]
const fn foo() {
    mir! {
        let val: i32;
        let _val2: i32;
        {
            StorageLive(val);
            val = 42;
            StorageLive(val);
            _val2 = val;
            Return()
        }
    }
}

fn main() {
    const { foo() };
}
