//@ compile-flags: -A unknown_tool::foo

fn main() {}

//~? ERROR unknown lint tool: `unknown_tool`
//~? ERROR unknown lint tool: `unknown_tool`
//~? ERROR unknown lint tool: `unknown_tool`

// ferrocene-annotations: um_rustc_A
