// Test that macro-expanded non-inline modules behave correctly

macro_rules! mod_decl {
    ($i:ident) => {
        mod $i; //~ ERROR cannot declare a non-inline module inside a block
    };
}

mod macro_expanded_mod_helper {
    mod_decl!(foo); // This should search in the folder `macro_expanded_mod_helper`
}

fn main() {
    mod_decl!(foo);
}

// ferrocene-annotations: fls_wjldgtio5o75
// Macro Expansion
//
// ferrocene-annotations: fls_vnvt40pa48n8
// Macro Invocation
//
// ferrocene-annotations: fls_e9hwvqsib5d5
// Modules
//
// ferrocene-annotations: fls_xa7lp0zg1ol2
// Declarative Macros
//
// ferrocene-annotations: fls_9kjpxri0axvg
// Weak Keywords
