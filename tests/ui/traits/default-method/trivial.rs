//@ run-pass


trait Cat {
    fn meow(&self) -> bool;
    fn scratch(&self) -> bool;
    fn purr(&self) -> bool { true }
}

impl Cat for isize {
    fn meow(&self) -> bool {
        self.scratch()
    }
    fn scratch(&self) -> bool {
        self.purr()
    }
}

pub fn main() {
    assert!(5.meow());
}

// ferrocene-annotations: fls_z7q8kbjwdc7g
// Method Call Expressions
//
// ferrocene-annotations: fls_wqazkzle0ix9
// Method Resolution
