//@ run-pass
#![allow(dead_code)]
#![allow(non_camel_case_types)]


struct cat<U> {
    info : Vec<U> ,
    meows : usize,

    how_hungry : isize,
}

impl<U> cat<U> {
    pub fn speak<T>(&mut self, stuff: Vec<T> ) {
        self.meows += stuff.len();
    }
    pub fn meow_count(&mut self) -> usize { self.meows }
}

fn cat<U>(in_x : usize, in_y : isize, in_info: Vec<U> ) -> cat<U> {
    cat {
        meows: in_x,
        how_hungry: in_y,
        info: in_info
    }
}

pub fn main() {
  let mut nyan : cat<isize> = cat::<isize>(52, 99, vec![9]);
  let mut kitty = cat(1000, 2, vec!["tabby".to_string()]);
  assert_eq!(nyan.how_hungry, 99);
  assert_eq!(kitty.how_hungry, 2);
  nyan.speak(vec![1,2,3]);
  assert_eq!(nyan.meow_count(), 55);
  kitty.speak(vec!["meow".to_string(), "mew".to_string(), "purr".to_string(), "chirp".to_string()]);
  assert_eq!(kitty.meow_count(), 1004);
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
