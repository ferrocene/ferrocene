//@ run-pass

struct StringBuffer {
    s: String,
}

impl StringBuffer {
    pub fn append(&mut self, v: &str) {
        self.s.push_str(v);
    }
}

fn to_string(sb: StringBuffer) -> String {
    sb.s
}

pub fn main() {
    let mut sb = StringBuffer {
        s: String::new(),
    };
    sb.append("Hello, ");
    sb.append("World!");
    let str = to_string(sb);
    assert_eq!(str, "Hello, World!");
}

// ferrocene-annotations: fls_18k3uajrgq5f
// Field Access Expressions
//
// ferrocene-annotations: fls_xcwfotmq2e5d
// Field Resolution
