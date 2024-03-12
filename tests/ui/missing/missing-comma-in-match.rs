//@ run-rustfix

fn main() {
    match &Some(3) {
        &None => 1
        &Some(2) => { 3 }
        //~^ ERROR expected one of `,`, `.`, `?`, `}`, or an operator, found `=>`
        //~| NOTE expected one of `,`, `.`, `?`, `}`, or an operator
        _ => 2
    };
}

// ferrocene-annotations: fls_e5td0fa92fay
// Match Expressions
