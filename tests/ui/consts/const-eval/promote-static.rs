// regression test for #67609.

//@ check-pass

static NONE: Option<String> = None;

static NONE_REF_REF: &&Option<String> = {
    let x = &&NONE;
    x
};

fn main() {
    println!("{:?}", NONE_REF_REF);
}

// ferrocene-annotations: fls_wttihxen35as
// Constant Promotion
