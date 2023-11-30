fn main() {
    extern "C" {
        static symbol: [usize]; //~ ERROR: the size for values of type
    }
    println!("{}", symbol[0]);
}

// ferrocene-annotations: fls_s4yt19sptl7d
// External Statics
