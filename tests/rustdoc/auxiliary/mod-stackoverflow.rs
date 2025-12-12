//@ compile-flags: -Cmetadata=aux
//@ edition: 2015

pub mod tree {
    pub use tree;
}

pub mod tree2 {
    pub mod prelude {
        pub use tree2;
    }
}
