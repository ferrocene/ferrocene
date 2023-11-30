fn add_float_to_integer(x: u8) -> f32 {
    x + 100.0 //~ ERROR cannot add `{float}` to `u8`
}

fn add_str_to_float(x: f64) -> f64 {
    x + "foo" //~ ERROR cannot add `&str` to `f64`
}

fn add_lvar_to_float(x: f64) -> f64 {
    let y = 3;
    x + y //~ ERROR cannot add `{integer}` to `f64`
}

fn subtract_float_from_integer(x: u8) -> f32 {
    x - 100.0 //~ ERROR cannot subtract `{float}` from `u8`
}

fn subtract_str_from_f64(x: f64) -> f64 {
    x - "foo" //~ ERROR cannot subtract `&str` from `f64`
}

fn subtract_lvar_from_f64(x: f64) -> f64 {
    let y = 3;
    x - y //~ ERROR cannot subtract `{integer}` from `f64`
}

fn multiply_integer_by_float(x: u8) -> f32 {
    x * 100.0 //~ ERROR cannot multiply `u8` by `{float}`
}

fn multiply_f64_by_str(x: f64) -> f64 {
    x * "foo" //~ ERROR cannot multiply `f64` by `&str`
}

fn multiply_f64_by_lvar(x: f64) -> f64 {
    let y = 3;
    x * y //~ ERROR cannot multiply `f64` by `{integer}`
}

fn divide_integer_by_float(x: u8) -> u8 {
    x / 100.0 //~ ERROR cannot divide `u8` by `{float}`
}

fn divide_f64_by_str(x: f64) -> f64 {
    x / "foo" //~ ERROR cannot divide `f64` by `&str`
}

fn divide_f64_by_lvar(x: f64) -> f64 {
    let y = 3;
    x / y //~ ERROR cannot divide `f64` by `{integer}`
}

fn main() {}

// ferrocene-annotations: fls_1k9mkv7rbezi
// Arithmetic Expressions
//
// ferrocene-annotations: fls_29tlg1vyqay2
// Float Literals
//
// ferrocene-annotations: fls_2ed4axpsy9u0
// Integer Literals
//
// ferrocene-annotations: fls_hucd52suu6it
// Simple String Literals
//
// ferrocene-annotations: fls_boyhlu5srp6u
// String Literals
//
// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
//
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
//
// ferrocene-annotations: fls_h0dvogc64tfh
// Literal Expressions
//
// ferrocene-annotations: fls_94a8v54bufn8
// Values
//
// ferrocene-annotations: fls_e7zgqroy2qxn
// Value Expressions
//
// ferrocene-annotations: fls_zfibijmf8qe1
// Arithmetic Overflow
