// This is not autofixable because we give extra suggestions to end the first expression with `;`.
fn foo(a: Option<u32>, b: Option<u32>) -> bool {
    if let Some(x) = a { true } else { false }
    //~^ ERROR mismatched types
    //~| ERROR mismatched types
    && //~ ERROR mismatched types
    if let Some(y) = a { true } else { false }
}

fn main() {}

// ferrocene-annotations: fls_1pg5ig740tg1
// Expression Statements
