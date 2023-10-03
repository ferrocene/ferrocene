// ignore-tidy-cr

pub fn main() {
    br"a"; //~ ERROR bare CR not allowed in raw string
    br"é";  //~ ERROR non-ASCII character in raw byte string literal
    br##~"a"~##;  //~ ERROR only `#` is allowed in raw string delimitation
}

// ferrocene-annotations: fls_jps9102q0qfi
// Raw Byte String Literals
