fn main() {
    let ok = "{{everything fine}}";
    let bad = "\{it is wrong\}";
    //~^  ERROR unknown character escape: `{`
    //~|  HELP if used in a formatting string, curly braces are escaped with `{{` and `}}`
    //~| ERROR unknown character escape: `}`
    //~| HELP if used in a formatting string, curly braces are escaped with `{{` and `}}`
}

// ferrocene-annotations: fls_hucd52suu6it
// Simple String Literals
