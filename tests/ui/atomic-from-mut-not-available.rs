<<<PULL-UPSTREAM>>> file deleted upstream; move the Ferrocene annotations if any, and delete this file
//@ only-x86
//@ only-linux

fn main() {
    core::sync::atomic::AtomicU64::from_mut(&mut 0u64);
    //~^ ERROR: no function or associated item named `from_mut` found for struct `AtomicU64`
}

// ferrocene-annotations: fls_vyc9vcuamlph
// Atomics
