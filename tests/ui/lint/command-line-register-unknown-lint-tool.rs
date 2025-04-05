//@ compile-flags: -A unknown_tool::foo

fn main() {}

<<<<<<< HEAD
// ferrocene-annotations: um_rustc_A
=======
//~? ERROR unknown lint tool: `unknown_tool`
//~? ERROR unknown lint tool: `unknown_tool`
//~? ERROR unknown lint tool: `unknown_tool`
>>>>>>> pull-upstream-temp--do-not-use-for-real-code
