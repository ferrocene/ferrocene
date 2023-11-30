struct Ret;
struct Obj;

impl Obj {
    fn func() -> Ret {
        Ret
    }
}

fn func() -> Ret {
    Ret
}

fn main() {
    Obj::func.x();
    //~^ ERROR no method named `x` found
    func.x();
    //~^ ERROR no method named `x` found
}

// ferrocene-annotations: fls_z7q8kbjwdc7g
// Method Call Expressions
//
// ferrocene-annotations: fls_wqazkzle0ix9
// Method Resolution
