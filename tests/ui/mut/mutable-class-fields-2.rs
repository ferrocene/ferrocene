struct Cat {
  meows : usize,

  how_hungry : isize,
}

impl Cat {
  pub fn eat(&self) {
    self.how_hungry -= 5; //~ ERROR cannot assign
  }

}

fn cat(in_x : usize, in_y : isize) -> Cat {
    Cat {
        meows: in_x,
        how_hungry: in_y
    }
}

fn main() {
  let nyan : Cat = cat(52, 99);
  nyan.eat();
}

// ferrocene-annotations: fls_v5x85lt5ulva
// References
//
// ferrocene-annotations: fls_a14slch83hzn
// Borrowing
//
// ferrocene-annotations: fls_290jmzfh7x4e
// Compound Assignment Expressions
//
// ferrocene-annotations: fls_18k3uajrgq5f
// Field Access Expressions
//
// ferrocene-annotations: fls_6ydylimiv553
// Place Expressions
//
// ferrocene-annotations: fls_izdv9i4spokw
// Operator Expressions
//
// ferrocene-annotations: fls_xcwfotmq2e5d
// Field Resolution
