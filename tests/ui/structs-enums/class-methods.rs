//@ run-pass
#![allow(non_camel_case_types)]


struct cat {
  meows : usize,

  how_hungry : isize,
}

impl cat {
    pub fn speak(&mut self) { self.meows += 1; }
    pub fn meow_count(&mut self) -> usize { self.meows }
}

fn cat(in_x: usize, in_y: isize) -> cat {
    cat {
        meows: in_x,
        how_hungry: in_y
    }
}

pub fn main() {
  let mut nyan: cat = cat(52, 99);
  let kitty = cat(1000, 2);
  assert_eq!(nyan.how_hungry, 99);
  assert_eq!(kitty.how_hungry, 2);
  nyan.speak();
  assert_eq!(nyan.meow_count(), 53);
}

// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_z7q8kbjwdc7g
// Method Call Expressions
//
// ferrocene-annotations: fls_18k3uajrgq5f
// Field Access Expressions
//
// ferrocene-annotations: fls_xcwfotmq2e5d
// Field Resolution
//
// ferrocene-annotations: fls_wqazkzle0ix9
// Method Resolution
