fn /// document
foo() {}
//~^^ ERROR expected identifier, found doc comment `/// document`

fn main() {
    foo();
}

// ferrocene-annotations: fls_q8l2jza7d9xa
// Comments
