trait Node {
    fn zomg();
}

trait Graph<N: Node> {
    fn nodes<'a, I: Iterator<Item=&'a N>>(&'a self) -> I
        where N: 'a;
}

impl<N: Node> Graph<N> for Vec<N> {
    fn nodes<'a, I: Iterator<Item=&'a N>>(&self) -> I
        where N: 'a
    {
        self.iter() //~ ERROR mismatched types
    }
}

struct Stuff;

impl Node for Stuff {
    fn zomg() {
        println!("zomg");
    }
}

fn iterate<N: Node, G: Graph<N>>(graph: &G) {
    for node in graph.iter() { //~ ERROR no method named `iter` found
        node.zomg();
    }
}

pub fn main() {
    let graph = Vec::new();

    graph.push(Stuff);

    iterate(graph); //~ ERROR mismatched types
}

// ferrocene-annotations: fls_85vx1qfa061i
// Traits
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_e1pgdlv81vul
// Implementation Conformance
//
// ferrocene-annotations: fls_vhpwge5123cm
// Generic Parameters
//
// ferrocene-annotations: fls_yqcygq3y6m5j
// Lifetimes
//
// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
//
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
