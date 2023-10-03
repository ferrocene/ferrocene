fn main() {
    let a = 42._; //~ ERROR expected identifier, found reserved identifier `_`
                  //~| ERROR `{integer}` is a primitive type and therefore doesn't have fields
}

// ferrocene-annotations: fls_29tlg1vyqay2
// Float Literals
