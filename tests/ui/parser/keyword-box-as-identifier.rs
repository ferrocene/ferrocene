fn main() {
    let box = 0;
    //~^ ERROR expected pattern, found `=`
    let box: bool;
    //~^ ERROR expected pattern, found `:`
    let mut box = 0;
    //~^ ERROR expected pattern, found `=`
    let (box,) = (0,);
    //~^ ERROR expected pattern, found `,`
}

// ferrocene-annotations: fls_lish33a1naw5
// Keywords
//
// ferrocene-annotations: fls_cbsgp6k0qa82
// Reserved Keywords
