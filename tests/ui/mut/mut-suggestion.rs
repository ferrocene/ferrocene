#[derive(Copy, Clone)]
struct S;

impl S {
    fn mutate(&mut self) {
    }
}

fn func(arg: S) {
    //~^ HELP consider changing this to be mutable
    //~| SUGGESTION mut
    arg.mutate();
    //~^ ERROR cannot borrow `arg` as mutable, as it is not declared as mutable
}

fn main() {
    let local = S;
    //~^ HELP consider changing this to be mutable
    //~| SUGGESTION mut
    local.mutate();
    //~^ ERROR cannot borrow `local` as mutable, as it is not declared as mutable
}

// ferrocene-annotations: fls_v5x85lt5ulva
// References
//
// ferrocene-annotations: fls_a14slch83hzn
// Borrowing
//
// ferrocene-annotations: fls_qcb1n9c0e5hz
// Functions
//
// ferrocene-annotations: fls_z7q8kbjwdc7g
// Method Call Expressions
//
// ferrocene-annotations: fls_wqazkzle0ix9
// Method Resolution
