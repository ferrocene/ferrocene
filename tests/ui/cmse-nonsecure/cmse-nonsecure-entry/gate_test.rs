//@ ignore-thumbv8m.main this target does support the ABI and that is tested in
// `tests/ui/explicit-tail-calls/unsupported-abi/cmse-nonsecure-call.rs`
// gate-test-cmse_nonsecure_entry

#[no_mangle]
pub extern "cmse-nonsecure-entry" fn entry_function(input: u32) -> u32 {
    //~^ ERROR: is not a supported ABI for the current target [E0570]
    //~| ERROR: ABI is experimental and subject to change [E0658]
    input + 6
}

fn main() {}
