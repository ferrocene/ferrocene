//@ compile-flags: --error-format=short

fn foo(_: u32) {}

fn main() {
    foo("Bonjour".to_owned());
    let x = 0u32;
    x.salut();
}

// ferrocene-annotations: um_rustc_error_format
