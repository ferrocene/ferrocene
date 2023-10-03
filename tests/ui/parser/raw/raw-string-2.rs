fn main() {
    let x = r###"here's a long string"# "# "##;
    //~^ ERROR unterminated raw string
}

// ferrocene-annotations: fls_usr6iuwpwqqh
// Raw String Literals
