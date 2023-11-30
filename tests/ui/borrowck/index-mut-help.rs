// When mutably indexing a type that implements `Index` but not `IndexMut`, a
// special 'help' message is added to the output.
use std::collections::HashMap;


fn main() {
    let mut map = HashMap::new();
    map.insert("peter", "23".to_string());

    map["peter"].clear();           //~ ERROR
    map["peter"] = "0".to_string(); //~ ERROR
    let _ = &mut map["peter"];      //~ ERROR
}

//
// ferrocene-annotations: fls_sxcr4aa098i6
// Array and Slice Indexing Expressions
