#[deny(unreachable_patterns)]

fn parse_data1(data: &[u8]) -> u32 {
    match data {
        b"" => 1,
        _ => 2,
    }
}

fn parse_data2(data: &[u8]) -> u32 {
    match data { //~ ERROR non-exhaustive patterns: `&[_, ..]` not covered
        b"" => 1,
    }
}

fn parse_data3(data: &[u8; 0]) -> u8 {
    match data {
        b"" => 1,
    }
}

fn parse_data4(data: &[u8]) -> u8 {
    match data { //~ ERROR non-exhaustive patterns
        b"aaa" => 0,
        [_, _, _] => 1,
    }
}

fn parse_data5(data: &[u8; 3]) -> u8 {
    match data {
        b"aaa" => 0,
        [_, _, _] => 1,
    }
}

fn main() {}

// ferrocene-annotations: fls_142vncdktbin
// Reference Types
//
// ferrocene-annotations: fls_vpbikb73dw4k
// Slice Types
//
// ferrocene-annotations: fls_e5td0fa92fay
// Match Expressions
//
// ferrocene-annotations: fls_fqaffyrjob7v
// Byte String Literals
//
// ferrocene-annotations: fls_msbaxfc09vkk
// Simple Byte String Literals
//
// ferrocene-annotations: fls_qte70mgzpras
// Slice Patterns
//
// ferrocene-annotations: fls_57ic33pwdvp3
// Slice Pattern Matching
//
// ferrocene-annotations: fls_qfsfnql1t7m
// Underscore Patterns
//
// ferrocene-annotations: fls_yc4xm4hrfyw7
// Underscore Pattern Matching
