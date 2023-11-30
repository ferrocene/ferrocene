fn foo()
where
    T: Send,
    //~^ cannot find type `T` in this scope
{
    let s = "abc".to_string();
}

fn main() {}

// ferrocene-annotations: fls_7nv8ualeaqe3
// Where Clauses
