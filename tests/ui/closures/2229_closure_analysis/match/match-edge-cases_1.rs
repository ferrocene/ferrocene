//@ run-pass
//@ edition:2021

const PATTERN_REF: &str = "Hello World";
const NUMBER: i32 = 30;
const NUMBER_POINTER: *const i32 = &NUMBER;

pub fn edge_case_ref(event: &str) {
    let _ = || {
        match event {
            PATTERN_REF => (),
            _ => (),
        };
    };
}

pub fn edge_case_str(event: String) {
    let _ = || {
        match event.as_str() {
            "hello" => (),
            _ => (),
        };
    };
}

pub fn edge_case_raw_ptr(event: *const i32) {
    let _ = || {
        match event {
            NUMBER_POINTER => (), //~WARN behave unpredictably
            //~| previously accepted
            _ => (),
        };
    };
}

pub fn edge_case_char(event: char) {
    let _ = || {
        match event {
            'a' => (),
            _ => (),
        };
    };
}

fn main() {}

// ferrocene-annotations: fls_2krxnq8q9ef1
// Literal Patterns
//
// ferrocene-annotations: fls_azzf1llv3wf
// Literal Pattern Matching
//
// ferrocene-annotations: fls_qfsfnql1t7m
// Wildcard Patterns
//
// ferrocene-annotations: fls_yc4xm4hrfyw7
// Wildcard Pattern Matching
//
// ferrocene-annotations: fls_uloyjbaso8pz
// Path Patterns
//
// ferrocene-annotations: fls_d44aflefat88
// Path Pattern Matching
