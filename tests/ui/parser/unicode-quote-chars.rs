fn main() {
    println!(“hello world”);
    //~^ ERROR unknown start of token: \u{201c}
    //~^^ HELP Unicode characters '“' (Left Double Quotation Mark) and '”' (Right Double Quotation Mark) look like '"' (Quotation Mark), but are not
    //~^^^ ERROR unknown start of token: \u{201d}
    //~^^^^ HELP Unicode character '”' (Right Double Quotation Mark) looks like '"' (Quotation Mark), but it is not
    //~^^^^^ ERROR expected `,`, found `world`
}

// ferrocene-annotations: fls_2i089jvv8j5g
// Character Set
//
// ferrocene-annotations: fls_hucd52suu6it
// Simple String Literals
