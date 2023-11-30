#[track_caller]
extern "C" fn f() {}
//~^^ ERROR `#[track_caller]` requires Rust ABI

extern "C" {
    #[track_caller]
    fn g();
    //~^^ ERROR `#[track_caller]` requires Rust ABI
}

fn main() {}

// ferrocene-annotations: fls_usgd0xlijoxv
// ABI
