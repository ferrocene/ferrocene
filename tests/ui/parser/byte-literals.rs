// ignore-tidy-tab

static FOO: u8 = b'\f';  //~ ERROR unknown byte escape

pub fn main() {
    b'\f';  //~ ERROR unknown byte escape
    b'\x0Z';  //~ ERROR invalid character in numeric character escape: `Z`
    b'	';  //~ ERROR byte constant must be escaped
    b''';  //~ ERROR byte constant must be escaped
    b'é';  //~ ERROR non-ASCII character in byte literal
    b'a  //~ ERROR unterminated byte constant [E0763]
}

// ferrocene-annotations: fls_2ifjqwnw03ms
// Byte Literals
//
// ferrocene-annotations: fls_h0dvogc64tfh
// Literal Expressions
//
// ferrocene-annotations: fls_e7zgqroy2qxn
// Value Expressions
