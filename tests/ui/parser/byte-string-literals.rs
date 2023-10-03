static FOO: &'static [u8] = b"\f";  //~ ERROR unknown byte escape

pub fn main() {
    b"\f";  //~ ERROR unknown byte escape
    b"\x0Z";  //~ ERROR invalid character in numeric character escape: `Z`
    b"é";  //~ ERROR non-ASCII character in byte string literal
    br##"é"##;  //~ ERROR non-ASCII character in raw byte string literal
    b"a  //~ ERROR unterminated double quote byte string
}

// ferrocene-annotations: fls_fqaffyrjob7v
// Byte String Literals
//
// ferrocene-annotations: fls_h0dvogc64tfh
// Literal Expressions
//
// ferrocene-annotations: fls_e7zgqroy2qxn
// Value Expressions
