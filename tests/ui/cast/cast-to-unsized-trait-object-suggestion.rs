fn main() {
    &1 as dyn Send; //~ ERROR cast to unsized
    Box::new(1) as dyn Send; //~ ERROR cast to unsized
}

// ferrocene-annotations: fls_eiw4by8z75di
// Send and Sync
