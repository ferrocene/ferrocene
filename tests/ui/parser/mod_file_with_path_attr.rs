//@ normalize-stderr-test: "not_a_real_file.rs:.*\(" -> "not_a_real_file.rs: $$FILE_NOT_FOUND_MSG ("

#[path = "not_a_real_file.rs"]
mod m; //~ ERROR not_a_real_file.rs

fn main() {
    assert_eq!(m::foo(), 10);
}

// ferrocene-annotations: fls_1zbaajz5prpn
// Attribute path
//
// ferrocene-annotations: fls_e9hwvqsib5d5
// Modules
