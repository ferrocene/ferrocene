//@ compile-flags: -Cmetadata=aux

pub mod tree {
    pub use crate::tree;
}

pub mod tree2 {
    pub mod prelude {
        pub use crate::tree2;
    }
}
