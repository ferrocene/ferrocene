fn main() {
    extern "C" {
        static symbol: [usize]; //~ ERROR: the size for values of type
    }
    println!("{}", symbol[0]);
    //~^ ERROR: extern static is unsafe
}

// ferrocene-annotations: fls_s4yt19sptl7d
// External Statics
